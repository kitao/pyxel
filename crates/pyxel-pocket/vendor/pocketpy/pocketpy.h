/*
 *  Copyright (c) 2025 blueloveTH
 *  Distributed Under The MIT License
 *  https://github.com/pocketpy/pocketpy
 */

#pragma once

// clang-format off

#define PK_VERSION				"2.0.6"
#define PK_VERSION_MAJOR            2
#define PK_VERSION_MINOR            0
#define PK_VERSION_PATCH            6

/*************** feature settings ***************/

// Reduce the startup memory usage for embedded systems
#ifndef PK_LOW_MEMORY_MODE          // can be overridden by cmake
#define PK_LOW_MEMORY_MODE          0
#endif

// Whether to compile os-related modules or not
#ifndef PK_ENABLE_OS                // can be overridden by cmake
#define PK_ENABLE_OS                1
#endif

// GC min threshold
#ifndef PK_GC_MIN_THRESHOLD         // can be overridden by cmake
    #if PK_LOW_MEMORY_MODE
        #define PK_GC_MIN_THRESHOLD     2048
    #else
        #define PK_GC_MIN_THRESHOLD     16384
    #endif
#endif

// Memory allocation functions
#ifndef PK_MALLOC
#define PK_MALLOC(size)             malloc(size)
#define PK_REALLOC(ptr, size)       realloc(ptr, size)
#define PK_FREE(ptr)                free(ptr)
#endif

// This is the maximum size of the value stack in py_TValue units
// The actual size in bytes equals `sizeof(py_TValue) * PK_VM_STACK_SIZE`
#ifndef PK_VM_STACK_SIZE            // can be overridden by cmake
    #if PK_LOW_MEMORY_MODE
        #define PK_VM_STACK_SIZE    2048
    #else
        #define PK_VM_STACK_SIZE    16384
    #endif
#endif

// This is the maximum number of local variables in a function
// (not recommended to change this)
#ifndef PK_MAX_CO_VARNAMES          // can be overridden by cmake
#define PK_MAX_CO_VARNAMES          64
#endif

/*************** internal settings ***************/
// This is the maximum character length of a module path
#define PK_MAX_MODULE_PATH_LEN      63

// This is some math constants
#define PK_M_PI                     3.1415926535897932384
#define PK_M_E                      2.7182818284590452354
#define PK_M_DEG2RAD                0.017453292519943295
#define PK_M_RAD2DEG                57.29577951308232

#ifdef _WIN32
    #define PK_PLATFORM_SEP '\\'
#else
    #define PK_PLATFORM_SEP '/'
#endif



// clang-format off

#if defined(_WIN32) || defined(_WIN64)
    #ifdef PY_DYNAMIC_MODULE
        #define PK_API __declspec(dllimport)
    #else
        #define PK_API __declspec(dllexport)
    #endif
    #define PK_EXPORT __declspec(dllexport)
    #define PY_SYS_PLATFORM     0
    #define PY_SYS_PLATFORM_STRING "win32"
#elif __EMSCRIPTEN__
    #define PK_API
    #define PK_EXPORT
    #define PY_SYS_PLATFORM     1
    #define PY_SYS_PLATFORM_STRING "emscripten"
#elif __APPLE__
    #include <TargetConditionals.h>
    #if TARGET_IPHONE_SIMULATOR
        // iOS, tvOS, or watchOS Simulator
        #define PY_SYS_PLATFORM     2
        #define PY_SYS_PLATFORM_STRING "ios"
    #elif TARGET_OS_IPHONE
        // iOS, tvOS, or watchOS device
        #define PY_SYS_PLATFORM     2
        #define PY_SYS_PLATFORM_STRING "ios"
    #elif TARGET_OS_MAC
        #define PY_SYS_PLATFORM     3
        #define PY_SYS_PLATFORM_STRING "darwin"
    #else
    #   error "Unknown Apple platform"
    #endif
    #define PK_API __attribute__((visibility("default")))
    #define PK_EXPORT __attribute__((visibility("default")))
#elif __ANDROID__
    #define PK_API __attribute__((visibility("default")))
    #define PK_EXPORT __attribute__((visibility("default")))
    #define PY_SYS_PLATFORM     4
    #define PY_SYS_PLATFORM_STRING "android"
#elif __linux__
    #define PK_API __attribute__((visibility("default")))
    #define PK_EXPORT __attribute__((visibility("default")))
    #define PY_SYS_PLATFORM     5
    #define PY_SYS_PLATFORM_STRING "linux"
#else
    #define PK_API
    #define PY_SYS_PLATFORM     6
    #define PY_SYS_PLATFORM_STRING "unknown"
#endif

#if PY_SYS_PLATFORM == 0 || PY_SYS_PLATFORM == 3 || PY_SYS_PLATFORM == 5
    #define PK_IS_DESKTOP_PLATFORM 1
#else
    #define PK_IS_DESKTOP_PLATFORM 0
#endif



#include <stdint.h>

typedef union c11_vec2i {
    struct { int x, y; };
    int data[2];
    int64_t _i64;
} c11_vec2i;

typedef union c11_vec3i {
    struct { int x, y, z; };
    int data[3];
} c11_vec3i;

typedef union c11_vec2 {
    struct { float x, y; };
    float data[2];
} c11_vec2;

typedef union c11_vec3 {
    struct { float x, y, z; };
    float data[3];
} c11_vec3;

typedef union c11_mat3x3 {
    struct {
        float _11, _12, _13;
        float _21, _22, _23;
        float _31, _32, _33;
    };

    float m[3][3];
    float data[9];
} c11_mat3x3;



#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/************* Public Types *************/

/// A opaque type that represents a python object. You cannot access its members directly.
typedef struct py_TValue py_TValue;
/// An integer that represents a python identifier. This is to achieve string pooling and fast name
/// resolution.
typedef uint16_t py_Name;
/// An integer that represents a python type. `0` is invalid.
typedef int16_t py_Type;
/// A 64-bit integer type. Corresponds to `int` in python.
typedef int64_t py_i64;
/// A 64-bit floating-point type. Corresponds to `float` in python.
typedef double py_f64;
/// A generic destructor function.
typedef void (*py_Dtor)(void*);

/// A string view type. It is helpful for passing strings which are not null-terminated.
typedef struct c11_sv {
    const char* data;
    int size;
} c11_sv;

/// A struct contains the callbacks of the VM.
typedef struct py_Callbacks {
    /// Used by `__import__` to load source code of a module.
    char* (*importfile)(const char*);
    /// Used by `print` to output a string.
    void (*print)(const char*);
    /// Used by `input` to get a character.
    int (*getchar)();
} py_Callbacks;

#define PY_RAISE
#define PY_RETURN

/// A generic reference to a python object.
typedef py_TValue* py_Ref;
/// A reference which has the same lifespan as the python object.
typedef py_TValue* py_ObjectRef;
/// A global reference which has the same lifespan as the VM.
typedef py_TValue* py_GlobalRef;
/// A specific location in the value stack of the VM.
typedef py_TValue* py_StackRef;
/// An item reference to a container object. It invalidates when the container is modified.
typedef py_TValue* py_ItemRef;
/// An output reference for returning a value.
typedef py_TValue* py_OutRef;

/// Native function signature.
/// @param argc number of arguments.
/// @param argv array of arguments. Use `py_arg(i)` macro to get the i-th argument.
/// @return `true` if the function is successful or `false` if an exception is raised.
typedef bool (*py_CFunction)(int argc, py_StackRef argv) PY_RAISE PY_RETURN;

/// Python compiler modes.
/// + `EXEC_MODE`: for statements.
/// + `EVAL_MODE`: for expressions.
/// + `SINGLE_MODE`: for REPL or jupyter notebook execution.
enum py_CompileMode { EXEC_MODE, EVAL_MODE, SINGLE_MODE };

/************* Global Setup *************/

/// Initialize pocketpy and the default VM.
PK_API void py_initialize();
/// Finalize pocketpy and free all VMs.
PK_API void py_finalize();
/// Get the current VM index.
PK_API int py_currentvm();
/// Switch to a VM.
/// @param index index of the VM ranging from 0 to 16 (exclusive). `0` is the default VM.
PK_API void py_switchvm(int index);
/// Reset the current VM.
PK_API void py_resetvm();
/// Get the current VM context. This is used for user-defined data.
PK_API void* py_getvmctx();
/// Set the current VM context. This is used for user-defined data.
PK_API void py_setvmctx(void* ctx);
/// Interrupt the current VM and raise a `KeyboardInterrupt` exception.
PK_API void py_interrupt();
/// Set `sys.argv`. Used for storing command-line arguments.
PK_API void py_sys_setargv(int argc, char** argv);
/// Setup the callbacks for the current VM.
PK_API py_Callbacks* py_callbacks();

/// Run a source string.
/// @param source source string.
/// @param filename filename (for error messages).
/// @param mode compile mode. Use `EXEC_MODE` for statements `EVAL_MODE` for expressions.
/// @param module target module. Use NULL for the main module.
/// @return `true` if the execution is successful or `false` if an exception is raised.
PK_API bool py_exec(const char* source,
                       const char* filename,
                       enum py_CompileMode mode,
                       py_Ref module) PY_RAISE PY_RETURN;

/// Evaluate a source string. Equivalent to `py_exec(source, "<string>", EVAL_MODE, module)`.
PK_API bool py_eval(const char* source, py_Ref module) PY_RAISE PY_RETURN;

/// Run a source string with smart interpretation.
/// Example:
/// `py_newstr(py_r0(), "abc");`
/// `py_newint(py_r1(), 123);`
/// `py_smartexec("print(_0, _1)", NULL, py_r0(), py_r1());`
/// `// "abc 123" will be printed`.
PK_API bool py_smartexec(const char* source, py_Ref module, ...) PY_RAISE PY_RETURN;
/// Evaluate a source string with smart interpretation.
/// Example:
/// `py_newstr(py_r0(), "abc");`
/// `py_smarteval("len(_)", NULL, py_r0());`
/// `int res = py_toint(py_retval());`
/// `// res will be 3`.
PK_API bool py_smarteval(const char* source, py_Ref module, ...) PY_RAISE PY_RETURN;

/// Compile a source string into a code object.
/// Use python's `exec()` or `eval()` to execute it.
PK_API bool py_compile(const char* source,
                          const char* filename,
                          enum py_CompileMode mode,
                          bool is_dynamic) PY_RAISE PY_RETURN;

/// Python equivalent to `globals()`.
PK_API void py_newglobals(py_OutRef);
/// Python equivalent to `locals()`.
PK_API void py_newlocals(py_OutRef);

/************* Values Creation *************/

/// A shorthand for `True`.
PK_API py_GlobalRef py_True();
/// A shorthand for `False`.
PK_API py_GlobalRef py_False();
/// A shorthand for `None`.
PK_API py_GlobalRef py_None();
/// A shorthand for `nil`. `nil` is not a valid python object.
PK_API py_GlobalRef py_NIL();

/// Create an `int` object.
PK_API void py_newint(py_OutRef, py_i64);
/// Create a `float` object.
PK_API void py_newfloat(py_OutRef, py_f64);
/// Create a `bool` object.
PK_API void py_newbool(py_OutRef, bool);
/// Create a `str` object from a null-terminated string (utf-8).
PK_API void py_newstr(py_OutRef, const char*);
/// Create a `str` object with `n` UNINITIALIZED bytes plus `'\0'`.
PK_API char* py_newstrn(py_OutRef, int);
/// Create a `str` object from a `c11_sv`.
PK_API void py_newstrv(py_OutRef, c11_sv);
/// Create a formatted `str` object.
PK_API void py_newfstr(py_OutRef, const char*, ...);
/// Create a `bytes` object with `n` UNINITIALIZED bytes.
PK_API unsigned char* py_newbytes(py_OutRef, int n);
/// Create a `None` object.
PK_API void py_newnone(py_OutRef);
/// Create a `NotImplemented` object.
PK_API void py_newnotimplemented(py_OutRef);
/// Create a `...` object.
PK_API void py_newellipsis(py_OutRef);
/// Create a `nil` object. `nil` is an invalid representation of an object.
/// Don't use it unless you know what you are doing.
PK_API void py_newnil(py_OutRef);
/// Create a `tuple` with `n` UNINITIALIZED elements.
/// You should initialize all elements before using it.
PK_API py_ObjectRef py_newtuple(py_OutRef, int n);
/// Create an empty `list`.
PK_API void py_newlist(py_OutRef);
/// Create a `list` with `n` UNINITIALIZED elements.
/// You should initialize all elements before using it.
PK_API void py_newlistn(py_OutRef, int n);
/// Create an empty `dict`.
PK_API void py_newdict(py_OutRef);
/// Create an UNINITIALIZED `slice` object.
/// You should use `py_setslot()` to set `start`, `stop`, and `step`.
PK_API void py_newslice(py_OutRef);
/// Create a `nativefunc` object.
PK_API void py_newnativefunc(py_OutRef, py_CFunction);
/// Create a `function` object.
PK_API py_Name py_newfunction(py_OutRef out,
                                 const char* sig,
                                 py_CFunction f,
                                 const char* docstring,
                                 int slots);
/// Create a `boundmethod` object.
PK_API void py_newboundmethod(py_OutRef out, py_Ref self, py_Ref func);

/************* Name Convertions *************/

/// Convert a null-terminated string to a name.
PK_API py_Name py_name(const char*);
/// Convert a name to a null-terminated string.
PK_API const char* py_name2str(py_Name);
/// Convert a name to a python `str` object with cache.
PK_API py_GlobalRef py_name2ref(py_Name);
/// Convert a `c11_sv` to a name.
PK_API py_Name py_namev(c11_sv);
/// Convert a name to a `c11_sv`.
PK_API c11_sv py_name2sv(py_Name);

#define py_ismagicname(name) (name <= __missing__)

/************* Meta Operations *************/

/// Create a new type.
/// @param name name of the type.
/// @param base base type.
/// @param module module where the type is defined. Use `NULL` for built-in types.
/// @param dtor destructor function. Use `NULL` if not needed.
PK_API py_Type py_newtype(const char* name,
                             py_Type base,
                             const py_GlobalRef module,
                             py_Dtor dtor);

/// Create a new object.
/// @param out output reference.
/// @param type type of the object.
/// @param slots number of slots. Use `-1` to create a `__dict__`.
/// @param udsize size of your userdata.
/// @return pointer to the userdata.
PK_API void* py_newobject(py_OutRef out, py_Type type, int slots, int udsize);

/************* Type Cast *************/

/// Convert an `int` object in python to `int64_t`.
PK_API py_i64 py_toint(py_Ref);
/// Convert a `float` object in python to `double`.
PK_API py_f64 py_tofloat(py_Ref);
/// Cast a `int` or `float` object in python to `double`.
/// If successful, return true and set the value to `out`.
/// Otherwise, return false and raise `TypeError`.
PK_API bool py_castfloat(py_Ref, py_f64* out) PY_RAISE;
/// 32-bit version of `py_castfloat`.
PK_API bool py_castfloat32(py_Ref, float* out) PY_RAISE;
/// Cast a `int` object in python to `int64_t`.
PK_API bool py_castint(py_Ref, py_i64* out) PY_RAISE;
/// Convert a `bool` object in python to `bool`.
PK_API bool py_tobool(py_Ref);
/// Convert a `type` object in python to `py_Type`.
PK_API py_Type py_totype(py_Ref);
/// Convert a `str` object in python to null-terminated string.
PK_API const char* py_tostr(py_Ref);
/// Convert a `str` object in python to char array.
PK_API const char* py_tostrn(py_Ref, int* size);
/// Convert a `str` object in python to `c11_sv`.
PK_API c11_sv py_tosv(py_Ref);
/// Convert a `bytes` object in python to char array.
PK_API unsigned char* py_tobytes(py_Ref, int* size);
/// Resize a `bytes` object. It can only be resized down.
PK_API void py_bytes_resize(py_Ref, int size);
/// Convert a user-defined object to its userdata.
PK_API void* py_touserdata(py_Ref);

#define py_isint(self) py_istype(self, tp_int)
#define py_isfloat(self) py_istype(self, tp_float)
#define py_isbool(self) py_istype(self, tp_bool)
#define py_isstr(self) py_istype(self, tp_str)
#define py_islist(self) py_istype(self, tp_list)
#define py_istuple(self) py_istype(self, tp_tuple)
#define py_isdict(self) py_istype(self, tp_dict)

#define py_isnil(self) py_istype(self, 0)
#define py_isnone(self) py_istype(self, tp_NoneType)

/// Get the type of the object.
PK_API py_Type py_typeof(py_Ref self);
/// Get type by module and name. e.g. `py_gettype("time", py_name("struct_time"))`.
/// Return `0` if not found.
PK_API py_Type py_gettype(const char* module, py_Name name);
/// Check if the object is exactly the given type.
PK_API bool py_istype(py_Ref, py_Type);
/// Check if the object is an instance of the given type.
PK_API bool py_isinstance(py_Ref obj, py_Type type);
/// Check if the derived type is a subclass of the base type.
PK_API bool py_issubclass(py_Type derived, py_Type base);

/// Get the magic method from the given type only.
/// The returned reference is always valid. However, its value may be `nil`.
PK_API py_GlobalRef py_tpgetmagic(py_Type type, py_Name name);
/// Search the magic method from the given type to the base type.
/// Return `NULL` if not found.
PK_API py_GlobalRef py_tpfindmagic(py_Type, py_Name name);
/// Search the name from the given type to the base type.
/// Return `NULL` if not found.
PK_API py_ItemRef py_tpfindname(py_Type, py_Name name);

/// Get the type object of the given type.
PK_API py_GlobalRef py_tpobject(py_Type type);
/// Get the type name.
PK_API const char* py_tpname(py_Type type);
/// Call a type to create a new instance.
PK_API bool py_tpcall(py_Type type, int argc, py_Ref argv) PY_RAISE PY_RETURN;

/// Check if the object is an instance of the given type exactly.
/// Raise `TypeError` if the check fails.
PK_API bool py_checktype(py_Ref self, py_Type type) PY_RAISE;

/// Check if the object is an instance of the given type or its subclass.
/// Raise `TypeError` if the check fails.
PK_API bool py_checkinstance(py_Ref self, py_Type type) PY_RAISE;

#define py_checkint(self) py_checktype(self, tp_int)
#define py_checkfloat(self) py_checktype(self, tp_float)
#define py_checkbool(self) py_checktype(self, tp_bool)
#define py_checkstr(self) py_checktype(self, tp_str)

/************* References *************/

/// Get the i-th register.
/// All registers are located in a contiguous memory.
PK_API py_GlobalRef py_getreg(int i);
/// Set the i-th register.
PK_API void py_setreg(int i, py_Ref val);

#define py_r0() py_getreg(0)
#define py_r1() py_getreg(1)
#define py_r2() py_getreg(2)
#define py_r3() py_getreg(3)
#define py_r4() py_getreg(4)
#define py_r5() py_getreg(5)
#define py_r6() py_getreg(6)
#define py_r7() py_getreg(7)

/// Get variable in the `__main__` module.
PK_API py_ItemRef py_getglobal(py_Name name);
/// Set variable in the `__main__` module.
PK_API void py_setglobal(py_Name name, py_Ref val);
/// Get variable in the `builtins` module.
PK_API py_ItemRef py_getbuiltin(py_Name name);

/// Equivalent to `*dst = *src`.
PK_API void py_assign(py_Ref dst, py_Ref src);
/// Get the last return value.
PK_API py_GlobalRef py_retval();

/// Get an item from the object's `__dict__`.
/// Return `NULL` if not found.
PK_API py_ItemRef py_getdict(py_Ref self, py_Name name);
/// Set an item to the object's `__dict__`.
PK_API void py_setdict(py_Ref self, py_Name name, py_Ref val);
/// Delete an item from the object's `__dict__`.
/// Return `true` if the deletion is successful.
PK_API bool py_deldict(py_Ref self, py_Name name);
/// Prepare an insertion to the object's `__dict__`.
PK_API py_ItemRef py_emplacedict(py_Ref self, py_Name name);
/// Apply a function to all items in the object's `__dict__`.
/// Return `true` if the function is successful for all items.
/// NOTE: Be careful if `f` modifies the object's `__dict__`.
PK_API bool
    py_applydict(py_Ref self, bool (*f)(py_Name name, py_Ref val, void* ctx), void* ctx) PY_RAISE;

/// Get the i-th slot of the object.
/// The object must have slots and `i` must be in valid range.
PK_API py_ObjectRef py_getslot(py_Ref self, int i);
/// Set the i-th slot of the object.
PK_API void py_setslot(py_Ref self, int i, py_Ref val);

/************* Inspection *************/

/// Get the current `function` object on the stack.
/// Return `NULL` if not available.
/// NOTE: This function should be placed at the beginning of your decl-based bindings.
PK_API py_StackRef py_inspect_currentfunction();
/// Get the current `module` object where the code is executed.
/// Return `NULL` if not available.
PK_API py_GlobalRef py_inspect_currentmodule();

/************* Bindings *************/

/// Bind a function to the object via "decl-based" style.
/// @param obj the target object.
/// @param sig signature of the function. e.g. `add(x, y)`.
/// @param f function to bind.
PK_API void py_bind(py_Ref obj, const char* sig, py_CFunction f);
/// Bind a method to type via "argc-based" style.
/// @param type the target type.
/// @param name name of the method.
/// @param f function to bind.
PK_API void py_bindmethod(py_Type type, const char* name, py_CFunction f);
/// Bind a static method to type via "argc-based" style.
/// @param type the target type.
/// @param name name of the method.
/// @param f function to bind.
PK_API void py_bindstaticmethod(py_Type type, const char* name, py_CFunction f);
/// Bind a function to the object via "argc-based" style.
/// @param obj the target object.
/// @param name name of the function.
/// @param f function to bind.
PK_API void py_bindfunc(py_Ref obj, const char* name, py_CFunction f);
/// Bind a property to type.
/// @param type the target type.
/// @param name name of the property.
/// @param getter getter function.
/// @param setter setter function. Use `NULL` if not needed.
PK_API void
    py_bindproperty(py_Type type, const char* name, py_CFunction getter, py_CFunction setter);

#define py_bindmagic(type, __magic__, f) py_newnativefunc(py_tpgetmagic((type), __magic__), (f))

#define PY_CHECK_ARGC(n)                                                                           \
    if(argc != n) return TypeError("expected %d arguments, got %d", n, argc)

#define PY_CHECK_ARG_TYPE(i, type)                                                                 \
    if(!py_checktype(py_arg(i), type)) return false

#define py_offset(p, i) ((py_Ref)((char*)p + ((i) << 4)))
#define py_arg(i) py_offset(argv, i)

/************* Python Equivalents *************/

/// Python equivalent to `getattr(self, name)`.
PK_API bool py_getattr(py_Ref self, py_Name name) PY_RAISE PY_RETURN;
/// Python equivalent to `setattr(self, name, val)`.
PK_API bool py_setattr(py_Ref self, py_Name name, py_Ref val) PY_RAISE;
/// Python equivalent to `delattr(self, name)`.
PK_API bool py_delattr(py_Ref self, py_Name name) PY_RAISE;
/// Python equivalent to `self[key]`.
PK_API bool py_getitem(py_Ref self, py_Ref key) PY_RAISE PY_RETURN;
/// Python equivalent to `self[key] = val`.
PK_API bool py_setitem(py_Ref self, py_Ref key, py_Ref val) PY_RAISE;
/// Python equivalent to `del self[key]`.
PK_API bool py_delitem(py_Ref self, py_Ref key) PY_RAISE;

/// Perform a binary operation.
/// The result will be set to `py_retval()`.
/// The stack remains unchanged after the operation.
PK_API bool py_binaryop(py_Ref lhs, py_Ref rhs, py_Name op, py_Name rop) PY_RAISE PY_RETURN;

#define py_binaryadd(lhs, rhs) py_binaryop(lhs, rhs, __add__, __radd__)
#define py_binarysub(lhs, rhs) py_binaryop(lhs, rhs, __sub__, __rsub__)
#define py_binarymul(lhs, rhs) py_binaryop(lhs, rhs, __mul__, __rmul__)
#define py_binarytruediv(lhs, rhs) py_binaryop(lhs, rhs, __truediv__, __rtruediv__)
#define py_binaryfloordiv(lhs, rhs) py_binaryop(lhs, rhs, __floordiv__, __rfloordiv__)
#define py_binarymod(lhs, rhs) py_binaryop(lhs, rhs, __mod__, __rmod__)
#define py_binarypow(lhs, rhs) py_binaryop(lhs, rhs, __pow__, __rpow__)

#define py_binarylshift(lhs, rhs) py_binaryop(lhs, rhs, __lshift__, 0)
#define py_binaryrshift(lhs, rhs) py_binaryop(lhs, rhs, __rshift__, 0)
#define py_binaryand(lhs, rhs) py_binaryop(lhs, rhs, __and__, 0)
#define py_binaryor(lhs, rhs) py_binaryop(lhs, rhs, __or__, 0)
#define py_binaryxor(lhs, rhs) py_binaryop(lhs, rhs, __xor__, 0)
#define py_binarymatmul(lhs, rhs) py_binaryop(lhs, rhs, __matmul__, 0)

/************* Stack Operations *************/

/// Get the i-th object from the top of the stack.
/// `i` should be negative, e.g. (-1) means TOS.
PK_API py_StackRef py_peek(int i);
/// Push the object to the stack.
PK_API void py_push(py_Ref src);
/// Push a `nil` object to the stack.
PK_API void py_pushnil();
/// Push a `None` object to the stack.
PK_API void py_pushnone();
/// Push a `py_Name` to the stack. This is used for keyword arguments.
PK_API void py_pushname(py_Name name);
/// Pop an object from the stack.
PK_API void py_pop();
/// Shrink the stack by n.
PK_API void py_shrink(int n);
/// Get a temporary variable from the stack.
PK_API py_StackRef py_pushtmp();
/// Get the unbound method of the object.
/// Assume the object is located at the top of the stack.
/// If return true:  `[self] -> [unbound, self]`.
/// If return false: `[self] -> [self]` (no change).
PK_API bool py_pushmethod(py_Name name);
/// Call a callable object via pocketpy's calling convention.
/// You need to prepare the stack using this form: `callable, self/nil, arg1, arg2, ..., k1, v1, k2, v2, ...`
/// `argc` is the number of positional arguments excluding `self`.
/// `kwargc` is the number of keyword arguments, i.e. the number of key-value pairs.
/// The result will be set to `py_retval()`.
/// The stack size will be reduced by `2 + argc + kwargc * 2`.
PK_API bool py_vectorcall(uint16_t argc, uint16_t kwargc) PY_RAISE PY_RETURN;
/// Evaluate an expression and push the result to the stack.
/// This function is used for testing.
PK_API bool py_pusheval(const char* expr, py_GlobalRef module) PY_RAISE;

/************* Modules *************/

/// Create a new module.
PK_API py_GlobalRef py_newmodule(const char* path);
/// Get a module by path.
PK_API py_GlobalRef py_getmodule(const char* path);
/// Reload an existing module.
PK_API bool py_importlib_reload(py_GlobalRef module) PY_RAISE PY_RETURN;

/// Import a module.
/// The result will be set to `py_retval()`.
/// -1: error, 0: not found, 1: success
PK_API int py_import(const char* path) PY_RAISE PY_RETURN;

/************* Errors *************/

/// Raise an exception by type and message. Always return false.
PK_API bool py_exception(py_Type type, const char* fmt, ...) PY_RAISE;
/// Raise an expection object. Always return false.
PK_API bool py_raise(py_Ref) PY_RAISE;
/// Print the current exception.
/// The exception will be set as handled.
PK_API void py_printexc();
/// Format the current exception and return a null-terminated string.
/// The result should be freed by the caller.
/// The exception will be set as handled.
PK_API char* py_formatexc();
/// Check if an exception is raised.
PK_API bool py_checkexc(bool ignore_handled);
/// Check if the exception is an instance of the given type.
/// This function is roughly equivalent to python's `except <T> as e:` block.
/// If match, the exception will be stored in `py_retval()` as handled.
PK_API bool py_matchexc(py_Type type) PY_RETURN;
/// Clear the current exception.
/// @param p0 the unwinding point. Use `NULL` if not needed.
PK_API void py_clearexc(py_StackRef p0);

#define NameError(n) py_exception(tp_NameError, "name '%n' is not defined", (n))
#define TypeError(...) py_exception(tp_TypeError, __VA_ARGS__)
#define RuntimeError(...) py_exception(tp_RuntimeError, __VA_ARGS__)
#define OSError(...) py_exception(tp_OSError, __VA_ARGS__)
#define ValueError(...) py_exception(tp_ValueError, __VA_ARGS__)
#define IndexError(...) py_exception(tp_IndexError, __VA_ARGS__)
#define ImportError(...) py_exception(tp_ImportError, __VA_ARGS__)
#define ZeroDivisionError(...) py_exception(tp_ZeroDivisionError, __VA_ARGS__)
#define AttributeError(self, n)                                                                    \
    py_exception(tp_AttributeError, "'%t' object has no attribute '%n'", (self)->type, (n))
#define UnboundLocalError(n)                                                                       \
    py_exception(tp_UnboundLocalError, "cannot access local variable '%n' where it is not associated with a value", (n))

PK_API bool StopIteration() PY_RAISE;
PK_API bool KeyError(py_Ref key) PY_RAISE;

/************* Operators *************/

/// Python equivalent to `bool(val)`.
/// 1: true, 0: false, -1: error
PK_API int py_bool(py_Ref val) PY_RAISE;
/// Compare two objects.
/// 1: lhs == rhs, 0: lhs != rhs, -1: error
PK_API int py_equal(py_Ref lhs, py_Ref rhs) PY_RAISE;
/// Compare two objects.
/// 1: lhs < rhs, 0: lhs >= rhs, -1: error
PK_API int py_less(py_Ref lhs, py_Ref rhs) PY_RAISE;

#define py_eq(lhs, rhs) py_binaryop(lhs, rhs, __eq__, __eq__)
#define py_ne(lhs, rhs) py_binaryop(lhs, rhs, __ne__, __ne__)
#define py_lt(lhs, rhs) py_binaryop(lhs, rhs, __lt__, __gt__)
#define py_le(lhs, rhs) py_binaryop(lhs, rhs, __le__, __ge__)
#define py_gt(lhs, rhs) py_binaryop(lhs, rhs, __gt__, __lt__)
#define py_ge(lhs, rhs) py_binaryop(lhs, rhs, __ge__, __le__)

/// Python equivalent to `callable(val)`.
PK_API bool py_callable(py_Ref val);
/// Get the hash value of the object.
PK_API bool py_hash(py_Ref, py_i64* out) PY_RAISE;
/// Get the iterator of the object.
PK_API bool py_iter(py_Ref) PY_RAISE PY_RETURN;
/// Get the next element from the iterator.
/// 1: success, 0: StopIteration, -1: error
PK_API int py_next(py_Ref) PY_RAISE PY_RETURN;
/// Python equivalent to `lhs is rhs`.
PK_API bool py_isidentical(py_Ref, py_Ref);
/// Call a function.
/// It prepares the stack and then performs a `vectorcall(argc, 0, false)`.
/// The result will be set to `py_retval()`.
/// The stack remains unchanged after the operation.
PK_API bool py_call(py_Ref f, int argc, py_Ref argv) PY_RAISE PY_RETURN;

#ifndef NDEBUG
/// Call a `py_CFunction` in a safe way.
/// This function does extra checks to help you debug `py_CFunction`.
PK_API bool py_callcfunc(py_CFunction f, int argc, py_Ref argv) PY_RAISE PY_RETURN;
#else
#define py_callcfunc(f, argc, argv) (f((argc), (argv)))
#endif

/// Python equivalent to `str(val)`.
PK_API bool py_str(py_Ref val) PY_RAISE PY_RETURN;
/// Python equivalent to `repr(val)`.
PK_API bool py_repr(py_Ref val) PY_RAISE PY_RETURN;
/// Python equivalent to `len(val)`.
PK_API bool py_len(py_Ref val) PY_RAISE PY_RETURN;
/// Python equivalent to `json.dumps(val)`.
PK_API bool py_json_dumps(py_Ref val) PY_RAISE PY_RETURN;
/// Python equivalent to `json.loads(val)`.
PK_API bool py_json_loads(const char* source) PY_RAISE PY_RETURN;
/// Python equivalent to `pickle.dumps(val)`.
PK_API bool py_pickle_dumps(py_Ref val) PY_RAISE PY_RETURN;
/// Python equivalent to `pickle.loads(val)`.
PK_API bool py_pickle_loads(const unsigned char* data, int size) PY_RAISE PY_RETURN;
/************* Unchecked Functions *************/

PK_API py_ObjectRef py_tuple_data(py_Ref self);
PK_API py_ObjectRef py_tuple_getitem(py_Ref self, int i);
PK_API void py_tuple_setitem(py_Ref self, int i, py_Ref val);
PK_API int py_tuple_len(py_Ref self);

PK_API py_ItemRef py_list_data(py_Ref self);
PK_API py_ItemRef py_list_getitem(py_Ref self, int i);
PK_API void py_list_setitem(py_Ref self, int i, py_Ref val);
PK_API void py_list_delitem(py_Ref self, int i);
PK_API int py_list_len(py_Ref self);
PK_API void py_list_swap(py_Ref self, int i, int j);
PK_API void py_list_append(py_Ref self, py_Ref val);
PK_API py_ItemRef py_list_emplace(py_Ref self);
PK_API void py_list_clear(py_Ref self);
PK_API void py_list_insert(py_Ref self, int i, py_Ref val);

/// -1: error, 0: not found, 1: found
PK_API int py_dict_getitem(py_Ref self, py_Ref key) PY_RAISE PY_RETURN;
/// true: success, false: error
PK_API bool py_dict_setitem(py_Ref self, py_Ref key, py_Ref val) PY_RAISE;
/// -1: error, 0: not found, 1: found (and deleted)
PK_API int py_dict_delitem(py_Ref self, py_Ref key) PY_RAISE;

/// -1: error, 0: not found, 1: found
PK_API int py_dict_getitem_by_str(py_Ref self, const char* key) PY_RAISE PY_RETURN;
/// -1: error, 0: not found, 1: found
PK_API int py_dict_getitem_by_int(py_Ref self, py_i64 key) PY_RAISE PY_RETURN;
/// true: success, false: error
PK_API bool py_dict_setitem_by_str(py_Ref self, const char* key, py_Ref val) PY_RAISE;
/// true: success, false: error
PK_API bool py_dict_setitem_by_int(py_Ref self, py_i64 key, py_Ref val) PY_RAISE;
/// -1: error, 0: not found, 1: found (and deleted)
PK_API int py_dict_delitem_by_str(py_Ref self, const char* key) PY_RAISE;
/// -1: error, 0: not found, 1: found (and deleted)
PK_API int py_dict_delitem_by_int(py_Ref self, py_i64 key) PY_RAISE;

/// true: success, false: error
PK_API bool
    py_dict_apply(py_Ref self, bool (*f)(py_Ref key, py_Ref val, void* ctx), void* ctx) PY_RAISE;
/// noexcept
PK_API int py_dict_len(py_Ref self);

/************* linalg module *************/
void py_newvec2(py_OutRef out, c11_vec2);
void py_newvec3(py_OutRef out, c11_vec3);
void py_newvec2i(py_OutRef out, c11_vec2i);
void py_newvec3i(py_OutRef out, c11_vec3i);
c11_mat3x3* py_newmat3x3(py_OutRef out);
c11_vec2 py_tovec2(py_Ref self);
c11_vec3 py_tovec3(py_Ref self);
c11_vec2i py_tovec2i(py_Ref self);
c11_vec3i py_tovec3i(py_Ref self);
c11_mat3x3* py_tomat3x3(py_Ref self);

/************* Others *************/

/// An utility function to read a line from stdin for REPL.
PK_API int py_replinput(char* buf, int max_size);

/// Python favored string formatting.
/// %d: int
/// %i: py_i64 (int64_t)
/// %f: py_f64 (double)
/// %s: const char*
/// %q: c11_sv
/// %v: c11_sv
/// %c: char
/// %p: void*
/// %t: py_Type
/// %n: py_Name

enum py_MagicNames {
    py_MagicNames__NULL,  // 0 is reserved

#define MAGIC_METHOD(x) x,
#ifdef MAGIC_METHOD

// math operators
MAGIC_METHOD(__lt__)
MAGIC_METHOD(__le__)
MAGIC_METHOD(__gt__)
MAGIC_METHOD(__ge__)
/////////////////////////////
MAGIC_METHOD(__neg__)
MAGIC_METHOD(__abs__)
MAGIC_METHOD(__float__)
MAGIC_METHOD(__int__)
MAGIC_METHOD(__round__)
MAGIC_METHOD(__divmod__)
/////////////////////////////
MAGIC_METHOD(__add__)
MAGIC_METHOD(__radd__)
MAGIC_METHOD(__sub__)
MAGIC_METHOD(__rsub__)
MAGIC_METHOD(__mul__)
MAGIC_METHOD(__rmul__)
MAGIC_METHOD(__truediv__)
MAGIC_METHOD(__rtruediv__)
MAGIC_METHOD(__floordiv__)
MAGIC_METHOD(__rfloordiv__)
MAGIC_METHOD(__mod__)
MAGIC_METHOD(__rmod__)
MAGIC_METHOD(__pow__)
MAGIC_METHOD(__rpow__)
MAGIC_METHOD(__matmul__)
MAGIC_METHOD(__lshift__)
MAGIC_METHOD(__rshift__)
MAGIC_METHOD(__and__)
MAGIC_METHOD(__or__)
MAGIC_METHOD(__xor__)
/////////////////////////////
MAGIC_METHOD(__repr__)
MAGIC_METHOD(__str__)
MAGIC_METHOD(__hash__)
MAGIC_METHOD(__len__)
MAGIC_METHOD(__iter__)
MAGIC_METHOD(__next__)
MAGIC_METHOD(__contains__)
MAGIC_METHOD(__bool__)
MAGIC_METHOD(__invert__)
/////////////////////////////
MAGIC_METHOD(__eq__)
MAGIC_METHOD(__ne__)
// indexer
MAGIC_METHOD(__getitem__)
MAGIC_METHOD(__setitem__)
MAGIC_METHOD(__delitem__)
// specials
MAGIC_METHOD(__new__)
MAGIC_METHOD(__init__)
MAGIC_METHOD(__call__)
MAGIC_METHOD(__enter__)
MAGIC_METHOD(__exit__)
MAGIC_METHOD(__name__)
MAGIC_METHOD(__all__)
MAGIC_METHOD(__package__)
MAGIC_METHOD(__path__)
MAGIC_METHOD(__class__)
MAGIC_METHOD(__getattr__)
MAGIC_METHOD(__reduce__)
MAGIC_METHOD(__missing__)

#endif
#undef MAGIC_METHOD
};

enum py_PredefinedTypes {
    tp_nil = 0,
    tp_object = 1,
    tp_type,  // py_Type
    tp_int,
    tp_float,
    tp_bool,
    tp_str,
    tp_str_iterator,
    tp_list,   // c11_vector
    tp_tuple,  // N slots
    tp_array_iterator,
    tp_slice,  // 3 slots (start, stop, step)
    tp_range,
    tp_range_iterator,
    tp_module,
    tp_function,
    tp_nativefunc,
    tp_boundmethod,    // 2 slots (self, func)
    tp_super,          // 1 slot + py_Type
    tp_BaseException,  // 2 slots (arg + inner_exc)
    tp_Exception,
    tp_bytes,
    tp_namedict,
    tp_locals,
    tp_code,
    tp_dict,
    tp_dict_items,    // 1 slot
    tp_property,      // 2 slots (getter + setter)
    tp_star_wrapper,  // 1 slot + int level
    tp_staticmethod,  // 1 slot
    tp_classmethod,   // 1 slot
    tp_NoneType,
    tp_NotImplementedType,
    tp_ellipsis,
    tp_generator,
    /* builtin exceptions */
    tp_SystemExit,
    tp_KeyboardInterrupt,
    tp_StopIteration,
    tp_SyntaxError,
    tp_StackOverflowError,
    tp_OSError,
    tp_NotImplementedError,
    tp_TypeError,
    tp_IndexError,
    tp_ValueError,
    tp_RuntimeError,
    tp_ZeroDivisionError,
    tp_NameError,
    tp_UnboundLocalError,
    tp_AttributeError,
    tp_ImportError,
    tp_AssertionError,
    tp_KeyError,
    /* linalg */
    tp_vec2,
    tp_vec3,
    tp_vec2i,
    tp_vec3i,
    tp_mat3x3,
    /* array2d */
    tp_array2d_like,
    tp_array2d_like_iterator,
    tp_array2d,
    tp_array2d_view,
    tp_chunked_array2d,
};

#ifdef __cplusplus
}
#endif
