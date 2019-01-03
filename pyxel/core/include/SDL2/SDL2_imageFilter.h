/*

SDL2_imageFilter.h: byte-image "filter" routines 

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

#ifndef _SDL2_imageFilter_h
#define _SDL2_imageFilter_h

/* Set up for C function definitions, even when using C++ */
#ifdef __cplusplus
extern "C" {
#endif

	/* ---- Function Prototypes */

#ifdef _MSC_VER
#  if defined(DLL_EXPORT) && !defined(LIBSDL2_GFX_DLL_IMPORT)
#    define SDL2_IMAGEFILTER_SCOPE __declspec(dllexport)
#  else
#    ifdef LIBSDL2_GFX_DLL_IMPORT
#      define SDL2_IMAGEFILTER_SCOPE __declspec(dllimport)
#    endif
#  endif
#endif
#ifndef SDL2_IMAGEFILTER_SCOPE
#  define SDL2_IMAGEFILTER_SCOPE extern
#endif

	/* Comments:                                                                           */
	/*  1.) MMX functions work best if all data blocks are aligned on a 32 bytes boundary. */
	/*  2.) Data that is not within an 8 byte boundary is processed using the C routine.   */
	/*  3.) Convolution routines do not have C routines at this time.                      */

	// Detect MMX capability in CPU
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterMMXdetect(void);

	// Force use of MMX off (or turn possible use back on)
	SDL2_IMAGEFILTER_SCOPE void SDL_imageFilterMMXoff(void);
	SDL2_IMAGEFILTER_SCOPE void SDL_imageFilterMMXon(void);

	//
	// All routines return:
	//   0   OK
	//  -1   Error (internal error, parameter error)
	//

	//  SDL_imageFilterAdd: D = saturation255(S1 + S2)
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterAdd(unsigned char *Src1, unsigned char *Src2, unsigned char *Dest, unsigned int length);

	//  SDL_imageFilterMean: D = S1/2 + S2/2
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterMean(unsigned char *Src1, unsigned char *Src2, unsigned char *Dest, unsigned int length);

	//  SDL_imageFilterSub: D = saturation0(S1 - S2)
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterSub(unsigned char *Src1, unsigned char *Src2, unsigned char *Dest, unsigned int length);

	//  SDL_imageFilterAbsDiff: D = | S1 - S2 |
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterAbsDiff(unsigned char *Src1, unsigned char *Src2, unsigned char *Dest, unsigned int length);

	//  SDL_imageFilterMult: D = saturation(S1 * S2)
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterMult(unsigned char *Src1, unsigned char *Src2, unsigned char *Dest, unsigned int length);

	//  SDL_imageFilterMultNor: D = S1 * S2   (non-MMX)
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterMultNor(unsigned char *Src1, unsigned char *Src2, unsigned char *Dest, unsigned int length);

	//  SDL_imageFilterMultDivby2: D = saturation255(S1/2 * S2)
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterMultDivby2(unsigned char *Src1, unsigned char *Src2, unsigned char *Dest,
		unsigned int length);

	//  SDL_imageFilterMultDivby4: D = saturation255(S1/2 * S2/2)
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterMultDivby4(unsigned char *Src1, unsigned char *Src2, unsigned char *Dest,
		unsigned int length);

	//  SDL_imageFilterBitAnd: D = S1 & S2
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterBitAnd(unsigned char *Src1, unsigned char *Src2, unsigned char *Dest, unsigned int length);

	//  SDL_imageFilterBitOr: D = S1 | S2
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterBitOr(unsigned char *Src1, unsigned char *Src2, unsigned char *Dest, unsigned int length);

	//  SDL_imageFilterDiv: D = S1 / S2   (non-MMX)
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterDiv(unsigned char *Src1, unsigned char *Src2, unsigned char *Dest, unsigned int length);

	//  SDL_imageFilterBitNegation: D = !S
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterBitNegation(unsigned char *Src1, unsigned char *Dest, unsigned int length);

	//  SDL_imageFilterAddByte: D = saturation255(S + C)
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterAddByte(unsigned char *Src1, unsigned char *Dest, unsigned int length, unsigned char C);

	//  SDL_imageFilterAddUint: D = saturation255(S + (uint)C)
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterAddUint(unsigned char *Src1, unsigned char *Dest, unsigned int length, unsigned int C);

	//  SDL_imageFilterAddByteToHalf: D = saturation255(S/2 + C)
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterAddByteToHalf(unsigned char *Src1, unsigned char *Dest, unsigned int length,
		unsigned char C);

	//  SDL_imageFilterSubByte: D = saturation0(S - C)
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterSubByte(unsigned char *Src1, unsigned char *Dest, unsigned int length, unsigned char C);

	//  SDL_imageFilterSubUint: D = saturation0(S - (uint)C)
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterSubUint(unsigned char *Src1, unsigned char *Dest, unsigned int length, unsigned int C);

	//  SDL_imageFilterShiftRight: D = saturation0(S >> N)
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterShiftRight(unsigned char *Src1, unsigned char *Dest, unsigned int length, unsigned char N);

	//  SDL_imageFilterShiftRightUint: D = saturation0((uint)S >> N)
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterShiftRightUint(unsigned char *Src1, unsigned char *Dest, unsigned int length, unsigned char N);

	//  SDL_imageFilterMultByByte: D = saturation255(S * C)
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterMultByByte(unsigned char *Src1, unsigned char *Dest, unsigned int length, unsigned char C);

	//  SDL_imageFilterShiftRightAndMultByByte: D = saturation255((S >> N) * C)
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterShiftRightAndMultByByte(unsigned char *Src1, unsigned char *Dest, unsigned int length,
		unsigned char N, unsigned char C);

	//  SDL_imageFilterShiftLeftByte: D = (S << N)
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterShiftLeftByte(unsigned char *Src1, unsigned char *Dest, unsigned int length,
		unsigned char N);

	//  SDL_imageFilterShiftLeftUint: D = ((uint)S << N)
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterShiftLeftUint(unsigned char *Src1, unsigned char *Dest, unsigned int length,
		unsigned char N);

	//  SDL_imageFilterShiftLeft: D = saturation255(S << N)
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterShiftLeft(unsigned char *Src1, unsigned char *Dest, unsigned int length, unsigned char N);

	//  SDL_imageFilterBinarizeUsingThreshold: D = S >= T ? 255:0
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterBinarizeUsingThreshold(unsigned char *Src1, unsigned char *Dest, unsigned int length,
		unsigned char T);

	//  SDL_imageFilterClipToRange: D = (S >= Tmin) & (S <= Tmax) 255:0
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterClipToRange(unsigned char *Src1, unsigned char *Dest, unsigned int length,
		unsigned char Tmin, unsigned char Tmax);

	//  SDL_imageFilterNormalizeLinear: D = saturation255((Nmax - Nmin)/(Cmax - Cmin)*(S - Cmin) + Nmin)
	SDL2_IMAGEFILTER_SCOPE int SDL_imageFilterNormalizeLinear(unsigned char *Src, unsigned char *Dest, unsigned int length, int Cmin,
		int Cmax, int Nmin, int Nmax);

	/* Ends C function definitions when using C++ */
#ifdef __cplusplus
}
#endif

#endif				/* _SDL_imageFilter_h */
