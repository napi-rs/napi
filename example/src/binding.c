#include <node_api.h>

#ifdef __cpluscplus
extern "C" {
#endif

#define FUNCTIONS_MAP(V)                                                      \
  V(initialize, napi_rs_cb_initialize)

#define V(name, func)                                                         \
  extern napi_value func(napi_env env, napi_callback_info info);

  FUNCTIONS_MAP(V);
#undef V

#ifdef __GNUC__
#define UNLIKELY(expr) __builtin_expect(!!(expr), 0)
#else
#define UNLIKELY(expr) (expr)
#endif

#define ASSERT_NAPI_OK(expr)                                                  \
  if (UNLIKELY((expr) != napi_ok)) {                                          \
    napi_throw_error(env, NULL, "Assertion failed: (" #expr ") != napi_ok");  \
    return NULL;                                                              \
  }

napi_value init_bindings(napi_env env, napi_value exports) {
#define V(name, func)                                                         \
  { #name, NULL, func, NULL, NULL, NULL, napi_default, NULL },

  napi_property_descriptor descriptors[] = {
    FUNCTIONS_MAP(V)
  };
#undef V

  napi_status status = napi_define_properties(
      env,
      exports,
      sizeof(descriptors) / sizeof(descriptors[0]),
      descriptors);

  ASSERT_NAPI_OK(status);

  return exports;
}

NAPI_MODULE(NODE_GYP_MODULE_NAME, init_bindings)

#ifdef __cpluscplus
}
#endif
