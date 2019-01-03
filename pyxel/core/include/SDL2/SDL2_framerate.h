/*

SDL2_framerate.h: framerate manager

Copyright (C) 2012-2014  Andreas Schiffler

This software is provided 'as-is', without any express or implied
warranty. In no event will the authors be held liable for any damages
arising from the use of this software.

Permission is granted to anyone to use this software for any purpose,
including commercial applications, and to alter it and redistribute it
freely, subject to the following restrictions:

1. The origin of this software must not be misrepresented; you must not
claim that you wrote the original software. If you use this software
in a product, an acknowledgment in the product documentation would be
appreciated but is not required.

2. Altered source versions must be plainly marked as such, and must not be
misrepresented as being the original software.

3. This notice may not be removed or altered from any source
distribution.

Andreas Schiffler -- aschiffler at ferzkopp dot net

*/

#ifndef _SDL2_framerate_h
#define _SDL2_framerate_h

/* Set up for C function definitions, even when using C++ */
#ifdef __cplusplus
extern "C" {
#endif

	/* --- */

#include "SDL.h"

	/* --------- Definitions */

	/*!
	\brief Highest possible rate supported by framerate controller in Hz (1/s).
	*/
#define FPS_UPPER_LIMIT		200

	/*!
	\brief Lowest possible rate supported by framerate controller in Hz (1/s).
	*/
#define FPS_LOWER_LIMIT		1

	/*!
	\brief Default rate of framerate controller in Hz (1/s).
	*/
#define FPS_DEFAULT		30

	/*! 
	\brief Structure holding the state and timing information of the framerate controller. 
	*/
	typedef struct {
		Uint32 framecount;
		float rateticks;
		Uint32 baseticks;
		Uint32 lastticks;
		Uint32 rate;
	} FPSmanager;

	/* ---- Function Prototypes */

#ifdef _MSC_VER
#  if defined(DLL_EXPORT) && !defined(LIBSDL2_GFX_DLL_IMPORT)
#    define SDL2_FRAMERATE_SCOPE __declspec(dllexport)
#  else
#    ifdef LIBSDL2_GFX_DLL_IMPORT
#      define SDL2_FRAMERATE_SCOPE __declspec(dllimport)
#    endif
#  endif
#endif
#ifndef SDL2_FRAMERATE_SCOPE
#  define SDL2_FRAMERATE_SCOPE extern
#endif

	/* Functions return 0 or value for sucess and -1 for error */

	SDL2_FRAMERATE_SCOPE void SDL_initFramerate(FPSmanager * manager);
	SDL2_FRAMERATE_SCOPE int SDL_setFramerate(FPSmanager * manager, Uint32 rate);
	SDL2_FRAMERATE_SCOPE int SDL_getFramerate(FPSmanager * manager);
	SDL2_FRAMERATE_SCOPE int SDL_getFramecount(FPSmanager * manager);
	SDL2_FRAMERATE_SCOPE Uint32 SDL_framerateDelay(FPSmanager * manager);

	/* --- */

	/* Ends C function definitions when using C++ */
#ifdef __cplusplus
}
#endif

#endif				/* _SDL2_framerate_h */
