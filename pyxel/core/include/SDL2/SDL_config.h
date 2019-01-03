#if defined(__x86_64__)
#	if defined(_WIN32)
#	    include "SDL_config_windows_amd64.h"
#   elif defined(__APPLE__)
#	    include "SDL_config_darwin_amd64.h"
#	else
#	    include "SDL_config_linux_amd64.h"
#	endif
#elif defined(__i386__)
#	if defined(_WIN32)
#	    include "SDL_config_windows_386.h"
#	else
#	    include "SDL_config_linux_386.h"
#	endif
#elif defined(__arm__)
#	include "SDL_config_linux_arm.h"
#elif defined(__ANDROID__)
#	include "SDL_config_android_arm.h"
#else
#	error "No ABI matched"
#endif
