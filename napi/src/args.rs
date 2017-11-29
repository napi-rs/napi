use env::NapiEnv;
use result::NapiResult;
use sys;

pub trait NapiArgs<'env>: Sized {
    fn from_cb_info(
        env: &'env NapiEnv,
        cb_info: sys::napi_callback_info,
    ) -> NapiResult<Self>;
}
