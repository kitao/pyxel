def setup_apis(module, lib):
    def btnp_wrapper(key, hold=0, period=0):
        return lib.btnp(key, hold, period)

    module.btn = lib.btn
    module.btnp = btnp_wrapper
    module.btnr = lib.btnr
    module.mouse = lib.mouse
