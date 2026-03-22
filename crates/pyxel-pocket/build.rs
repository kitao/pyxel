use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

const POCKETPY_VERSION: &str = "2.1.8";

fn download(url: &str, dest: &str) {
    let status = Command::new("curl")
        .args(["-Lo", dest, url])
        .status()
        .expect("Failed to execute curl");
    assert!(status.success(), "Failed to download {url}");
}

/// Apply text-based patches to pocketpy.c for Python compatibility
fn patch_pocketpy(path: &str) {
    let mut src = fs::read_to_string(path).unwrap();

    // 1. Allow nested ternary expressions without parentheses
    //    Remove the ambiguity check that rejects `a if x else b if y else c`
    src = src.replace(
        concat!(
            "    if(e->cond->vt->is_ternary || e->false_expr->vt->is_ternary || e->true_expr->vt->is_ternary) {\n",
            "        return SyntaxError(self, \"nested ternary expressions without `()` are ambiguous\");\n",
            "    }\n",
        ),
        "    // [pyxel-pocket] nested ternary allowed\n",
    );

    // 2. Add format specifiers: x, X, o, b
    //    Extend the type switch to recognize these format chars
    src = src.replace(
        concat!(
            "        case 'f':\n",
            "        case 'd':\n",
            "        case 's':\n",
            "            type = spec.data[spec.size - 1];\n",
            "            spec.size--;  // remove last char\n",
            "            break;\n",
            "        default: type = ' '; break;\n",
        ),
        concat!(
            "        case 'f':\n",
            "        case 'd':\n",
            "        case 's':\n",
            "        case 'x':\n",
            "        case 'X':\n",
            "        case 'o':\n",
            "        case 'b':\n",
            "            type = spec.data[spec.size - 1];\n",
            "            spec.size--;\n",
            "            break;\n",
            "        default: type = ' '; break;\n",
        ),
    );
    //    Fix width parsing for empty spec (e.g. {:o} with no width)
    src = src.replace(
        concat!(
            "    } else {\n",
            "        // {10s}\n",
            "        IntParsingResult res = c11__parse_uint(spec, &width, 10);\n",
            "        if(res != IntParsing_SUCCESS) return ValueError(\"invalid format specifier\");\n",
            "        precision = -1;\n",
            "    }\n",
        ),
        concat!(
            "    } else {\n",
            "        // {10s} or just {s}\n",
            "        if(spec.size == 0) {\n",
            "            width = -1;\n",
            "        } else {\n",
            "            IntParsingResult res = c11__parse_uint(spec, &width, 10);\n",
            "            if(res != IntParsing_SUCCESS) return ValueError(\"invalid format specifier\");\n",
            "        }\n",
            "        precision = -1;\n",
            "    }\n",
        ),
    );
    //    Add output handlers for x/X/o/b after the 'd' handler
    src = src.replace(
        concat!(
            "        c11_sbuf__write_i64(&buf, py_toint(val));\n",
            "    } else if(type == 's') {\n",
        ),
        concat!(
            "        c11_sbuf__write_i64(&buf, py_toint(val));\n",
            "    } else if(type == 'x' || type == 'X') {\n",
            "        if(!py_checkint(val)) { c11_sbuf__dtor(&buf); return false; }\n",
            "        char _xbuf[32]; snprintf(_xbuf, sizeof(_xbuf), type == 'x' ? \"%llx\" : \"%llX\", (long long)py_toint(val));\n",
            "        c11_sbuf__write_cstr(&buf, _xbuf);\n",
            "    } else if(type == 'o') {\n",
            "        if(!py_checkint(val)) { c11_sbuf__dtor(&buf); return false; }\n",
            "        char _obuf[32]; snprintf(_obuf, sizeof(_obuf), \"%llo\", (long long)py_toint(val));\n",
            "        c11_sbuf__write_cstr(&buf, _obuf);\n",
            "    } else if(type == 'b') {\n",
            "        if(!py_checkint(val)) { c11_sbuf__dtor(&buf); return false; }\n",
            "        py_i64 _bval = py_toint(val); char _bbuf[66]; int _bi = 0;\n",
            "        if(_bval == 0) { _bbuf[_bi++] = '0'; }\n",
            "        else { py_i64 _bv = _bval < 0 ? -_bval : _bval; while(_bv > 0) { _bbuf[_bi++] = '0' + (_bv & 1); _bv >>= 1; } if(_bval < 0) _bbuf[_bi++] = '-'; }\n",
            "        for(int _bj = 0; _bj < _bi / 2; _bj++) { char _bt = _bbuf[_bj]; _bbuf[_bj] = _bbuf[_bi-1-_bj]; _bbuf[_bi-1-_bj] = _bt; }\n",
            "        _bbuf[_bi] = '\\0'; c11_sbuf__write_cstr(&buf, _bbuf);\n",
            "    } else if(type == 's') {\n",
        ),
    );

    // 3. Implicit string literal concatenation: "a" "b" -> "ab"
    //    In the parser's exprLiteral, after parsing one string, loop to consume adjacent strings
    src = src.replace(
        concat!(
            "static Error* exprLiteral(Compiler* self) {\n",
            "    LiteralExpr* e = LiteralExpr__new(prev()->line, &prev()->value);\n",
            "    Ctx__s_push(ctx(), (Expr*)e);\n",
            "    return NULL;\n",
            "}\n",
        ),
        concat!(
            "static Error* exprLiteral(Compiler* self) {\n",
            "    /* [pyxel-pocket] implicit string concatenation */\n",
            "    if(prev()->value.index == TokenValue_STR && curr()->type == TK_STR) {\n",
            "        c11_sbuf _cat; c11_sbuf__ctor(&_cat);\n",
            "        c11_sbuf__write_cstr(&_cat, prev()->value._str->data);\n",
            "        while(curr()->type == TK_STR) {\n",
            "            advance();\n",
            "            c11_sbuf__write_cstr(&_cat, prev()->value._str->data);\n",
            "        }\n",
            "        c11_string* merged = c11_sbuf__submit(&_cat);\n",
            "        TokenValue tv = {TokenValue_STR, ._str = merged};\n",
            "        LiteralExpr* e = LiteralExpr__new(prev()->line, &tv);\n",
            "        Ctx__s_push(ctx(), (Expr*)e);\n",
            "        return NULL;\n",
            "    }\n",
            "    LiteralExpr* e = LiteralExpr__new(prev()->line, &prev()->value);\n",
            "    Ctx__s_push(ctx(), (Expr*)e);\n",
            "    return NULL;\n",
            "}\n",
        ),
    );

    // 4. Nested list/set/dict comprehensions: [x for a in b for c in d]
    //    Make consume_comp loop on additional 'for' clauses
    src = src.replace(
        concat!(
            "static Error* consume_comp(Compiler* self, Opcode op0, Opcode op1) {\n",
            "    // [expr]\n",
            "    Error* err;\n",
            "    int line = prev()->line;\n",
            "    bool has_cond = false;\n",
            "    check(EXPR_VARS(self));  // [expr, vars]\n",
            "    consume(TK_IN);\n",
            "    check(parse_expression(self, PREC_TERNARY + 1, false));  // [expr, vars, iter]\n",
            "    if(match(TK_IF)) {\n",
            "        check(parse_expression(self, PREC_TERNARY + 1, false));  // [expr, vars, iter, cond]\n",
            "        has_cond = true;\n",
            "    }\n",
            "    CompExpr* ce = CompExpr__new(line, op0, op1);\n",
            "    if(has_cond) ce->cond = Ctx__s_popx(ctx());\n",
            "    ce->iter = Ctx__s_popx(ctx());\n",
            "    ce->vars = Ctx__s_popx(ctx());\n",
            "    ce->expr = Ctx__s_popx(ctx());\n",
            "    Ctx__s_push(ctx(), (Expr*)ce);\n",
            "    return NULL;\n",
            "}\n",
        ),
        concat!(
            "static Error* consume_comp(Compiler* self, Opcode op0, Opcode op1) {\n",
            "    // [pyxel-pocket] supports nested comprehensions: [x for a in b for c in d]\n",
            "    Error* err;\n",
            "    int line = prev()->line;\n",
            "    bool has_cond = false;\n",
            "    check(EXPR_VARS(self));\n",
            "    consume(TK_IN);\n",
            "    check(parse_expression(self, PREC_TERNARY + 1, false));\n",
            "    if(match(TK_IF)) {\n",
            "        check(parse_expression(self, PREC_TERNARY + 1, false));\n",
            "        has_cond = true;\n",
            "    }\n",
            "    CompExpr* ce = CompExpr__new(line, op0, op1);\n",
            "    if(has_cond) ce->cond = Ctx__s_popx(ctx());\n",
            "    ce->iter = Ctx__s_popx(ctx());\n",
            "    ce->vars = Ctx__s_popx(ctx());\n",
            "    ce->expr = Ctx__s_popx(ctx());\n",
            "    Ctx__s_push(ctx(), (Expr*)ce);\n",
            "    /* handle nested for clauses by wrapping in another CompExpr */\n",
            "    while(match(TK_FOR)) {\n",
            "        Expr* inner = Ctx__s_popx(ctx());\n",
            "        check(EXPR_VARS(self));\n",
            "        consume(TK_IN);\n",
            "        check(parse_expression(self, PREC_TERNARY + 1, false));\n",
            "        bool has_cond2 = false;\n",
            "        if(match(TK_IF)) {\n",
            "            check(parse_expression(self, PREC_TERNARY + 1, false));\n",
            "            has_cond2 = true;\n",
            "        }\n",
            "        CompExpr* outer = CompExpr__new(line, op0, op1);\n",
            "        outer->expr = inner;\n",
            "        /* swap: inner comp becomes the expr of outer, outer iterates */\n",
            "        if(has_cond2) outer->cond = Ctx__s_popx(ctx());\n",
            "        outer->iter = Ctx__s_popx(ctx());\n",
            "        outer->vars = Ctx__s_popx(ctx());\n",
            "        /* the inner comprehension's op should be the append op */\n",
            "        ((CompExpr*)inner)->op0 = OP_BUILD_LIST; /* dummy, won't emit */\n",
            "        ((CompExpr*)inner)->op1 = op1;\n",
            "        Ctx__s_push(ctx(), (Expr*)outer);\n",
            "    }\n",
            "    return NULL;\n",
            "}\n",
        ),
    );

    // 5. Tuple unpacking in for-loop targets: for i, (x, y) in ...
    //    Extend EXPR_VARS to handle parenthesized sub-patterns
    src = src.replace(
        concat!(
            "static Error* EXPR_VARS(Compiler* self) {\n",
            "    int count = 0;\n",
            "    do {\n",
            "        consume(TK_ID);\n",
            "        py_Name name = py_namev(Token__sv(prev()));\n",
            "        NameExpr* e = NameExpr__new(prev()->line, name, name_scope(self));\n",
            "        Ctx__s_push(ctx(), (Expr*)e);\n",
            "        count += 1;\n",
            "    } while(match(TK_COMMA));\n",
        ),
        concat!(
            "static Error* EXPR_VARS(Compiler* self) {\n",
            "    int count = 0;\n",
            "    do {\n",
            "        if(match(TK_LPAREN)) {\n",
            "            /* [pyxel-pocket] nested tuple unpacking: (x, y) */\n",
            "            int sub_count = 0;\n",
            "            do {\n",
            "                consume(TK_ID);\n",
            "                py_Name sname = py_namev(Token__sv(prev()));\n",
            "                NameExpr* se = NameExpr__new(prev()->line, sname, name_scope(self));\n",
            "                Ctx__s_push(ctx(), (Expr*)se);\n",
            "                sub_count += 1;\n",
            "            } while(match(TK_COMMA));\n",
            "            consume(TK_RPAREN);\n",
            "            if(sub_count > 1) {\n",
            "                SequenceExpr* sub = TupleExpr__new(prev()->line, sub_count);\n",
            "                for(int si = sub_count - 1; si >= 0; si--) sub->items[si] = Ctx__s_popx(ctx());\n",
            "                Ctx__s_push(ctx(), (Expr*)sub);\n",
            "            }\n",
            "        } else {\n",
            "            consume(TK_ID);\n",
            "            py_Name name = py_namev(Token__sv(prev()));\n",
            "            NameExpr* e = NameExpr__new(prev()->line, name, name_scope(self));\n",
            "            Ctx__s_push(ctx(), (Expr*)e);\n",
            "        }\n",
            "        count += 1;\n",
            "    } while(match(TK_COMMA));\n",
        ),
    );

    fs::write(path, src).unwrap();
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let pocketpy_dir = format!("{out_dir}/pocketpy-{POCKETPY_VERSION}");

    // Download amalgamated files
    if !Path::new(&pocketpy_dir).exists() {
        fs::create_dir_all(&pocketpy_dir).unwrap();
        let base_url =
            format!("https://github.com/pocketpy/pocketpy/releases/download/v{POCKETPY_VERSION}");
        download(
            &format!("{base_url}/pocketpy.c"),
            &format!("{pocketpy_dir}/pocketpy.c"),
        );
        download(
            &format!("{base_url}/pocketpy.h"),
            &format!("{pocketpy_dir}/pocketpy.h"),
        );

        // Apply patches
        patch_pocketpy(&format!("{pocketpy_dir}/pocketpy.c"));
    }

    // Build static library
    cc::Build::new()
        .file(format!("{pocketpy_dir}/pocketpy.c"))
        .include(&pocketpy_dir)
        .std("c11")
        .define("NDEBUG", None)
        .warnings(false)
        .compile("pocketpy");

    // Generate Rust FFI bindings
    let bindings = bindgen::Builder::default()
        .header(format!("{pocketpy_dir}/pocketpy.h"))
        .allowlist_function("py_.*")
        .allowlist_type("py_.*")
        .allowlist_var("py_.*|tp_.*|PY_.*")
        .allowlist_function("KeyError|StopIteration|TypeError")
        .use_core()
        .generate_comments(false)
        .generate()
        .expect("Failed to generate bindings");

    bindings
        .write_to_file(Path::new(&out_dir).join("bindings.rs"))
        .unwrap();
}
