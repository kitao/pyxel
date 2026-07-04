import importlib.util
from importlib.machinery import SourceFileLoader
from pathlib import Path

# format_prose is an extensionless script under scripts/, so load it via an
# explicit source loader (spec_from_file_location needs a .py suffix).
_MODULE_PATH = Path(__file__).parent.parent.parent / "scripts" / "format_prose"
_loader = SourceFileLoader("format_prose", str(_MODULE_PATH))
_spec = importlib.util.spec_from_loader(_loader.name, _loader)
assert _spec is not None
format_prose = importlib.util.module_from_spec(_spec)
_loader.exec_module(format_prose)


class TestJapaneseSpacing:
    def test_inserts_space_between_alphanumeric_and_kana(self):
        assert format_prose.format_text("Pyxelで", "ja") == "Pyxel で"

    def test_inserts_space_between_kana_and_alphanumeric(self):
        assert format_prose.format_text("画面640", "ja") == "画面 640"

    def test_keeps_middle_dot_separated_acronyms(self):
        assert format_prose.format_text("URL・QRコード", "ja") == "URL・QR コード"


class TestChineseSpacing:
    def test_inserts_space_between_hanzi_and_alphanumeric(self):
        assert format_prose.format_text("搜索API", "cn") == "搜索 API"


class TestTechnicalLoanwords:
    def test_keeps_short_technical_loanword(self):
        assert format_prose.format_text("ブラウザで確認", "ja") == "ブラウザで確認"
        assert (
            format_prose.format_text("コンストラクタで初期化", "ja")
            == "コンストラクタで初期化"
        )

    def test_keeps_existing_long_vowel(self):
        assert format_prose.format_text("ユーザーで確認", "ja") == "ユーザーで確認"


class TestLanguageAgnostic:
    def test_converts_fullwidth_alphanumeric_to_halfwidth(self):
        assert format_prose.format_text("ＡＢＣ０", "ja") == "ABC0"

    def test_removes_fullwidth_space(self):
        assert format_prose.format_text("a　b", "ja") == "a b"

    def test_collapses_consecutive_spaces(self):
        assert format_prose.format_text("a  b", "ja") == "a b"


class TestCodeSpan:
    def test_spaces_around_backtick_span(self):
        assert format_prose.format_text("`pyxel`コマンド", "ja") == "`pyxel` コマンド"

    def test_spaces_around_brace_span(self):
        assert format_prose.format_text("{ }内を指定", "ja") == "{ } 内を指定"

    def test_preserves_code_span_interior(self):
        assert format_prose.format_text("`ＡＢＣ`の例", "ja") == "`ＡＢＣ` の例"

    def test_preserves_markdown_link_target(self):
        text = "「[Pyxel MML の使い方](#api-仕様と使い方)」を参照"
        assert format_prose.format_text(text, "ja") == text

    def test_preserves_markdown_link_target_after_fullwidth_parentheses(self):
        text = "「[Pyxel MML の使い方]（#api-仕様と使い方）」を参照"
        assert format_prose.format_text(text, "ja") == text

    def test_preserves_keyboard_shortcut_parentheses(self):
        text = "Alt(Option)+2で保存"
        assert format_prose.format_text(text, "ja") == "Alt(Option)+2 で保存"

    def test_preserves_api_call_parentheses(self):
        assert (
            format_prose.format_text("pyxel.run()が呼ばれる", "ja")
            == "pyxel.run() が呼ばれる"
        )

    def test_preserves_api_call_inside_code_span(self):
        assert (
            format_prose.format_text("`pyxel.run ()`の例", "ja")
            == "`pyxel.run ()` の例"
        )

    def test_repairs_api_call_spacing(self):
        assert (
            format_prose.format_text("length () より軽い", "ja") == "length() より軽い"
        )

    def test_preserves_function_call_token(self):
        assert (
            format_prose.format_text("画面をclip()で戻す", "ja")
            == "画面を clip() で戻す"
        )

    def test_repairs_api_call_spacing_flush_against_kana(self):
        assert (
            format_prose.format_text("画面をclip ()で戻す", "ja")
            == "画面を clip() で戻す"
        )


class TestParentheses:
    def test_halfwidth_for_ascii_content(self):
        assert format_prose.format_text("リスト(0-2)です", "ja") == "リスト (0-2) です"

    def test_fullwidth_for_japanese_content(self):
        assert (
            format_prose.format_text("バンク(イメージ)を使う", "ja")
            == "バンク（イメージ）を使う"
        )

    def test_chinese_uses_halfwidth(self):
        assert format_prose.format_text("列表（0-2）", "cn") == "列表 (0-2)"

    def test_spaces_between_alphanumeric_and_parentheses(self):
        assert format_prose.format_text("Tone（音色）", "cn") == "Tone (音色)"


class TestLineBreak:
    def test_removes_space_after_line_break(self):
        assert (
            format_prose.format_text("前文。\n 在 `{ }` 内", "cn")
            == "前文。\n在 `{ }` 内"
        )

    def test_removes_space_after_escaped_line_break(self):
        assert (
            format_prose.format_text(r"前文。\n 在 `{ }` 内", "cn")
            == r"前文。\n在 `{ }` 内"
        )


class TestHalfwidthKana:
    def test_converts_halfwidth_kana_to_fullwidth(self):
        assert format_prose.format_text("ｱｲｳ", "ja") == "アイウ"


class TestJsonFormatting:
    def test_formats_only_target_language_values(self, tmp_path):
        path = tmp_path / "page.json"
        path.write_text(
            '{\n  "en": "検索API",\n  "ja": "検索API"\n}\n',
            encoding="utf-8",
        )

        format_prose._format_json(path)

        assert (
            path.read_text(encoding="utf-8")
            == '{\n  "en": "検索API",\n  "ja": "検索 API"\n}\n'
        )

    def test_formats_repeated_target_language_values(self, tmp_path):
        path = tmp_path / "page.json"
        path.write_text(
            '{\n  "items": [\n    {"ja": "画面640"},\n    {"ja": "画面640"}\n  ]\n}\n',
            encoding="utf-8",
        )

        format_prose._format_json(path)

        assert (
            path.read_text(encoding="utf-8")
            == '{\n  "items": [\n    {"ja": "画面 640"},\n    {"ja": "画面 640"}\n  ]\n}\n'
        )
