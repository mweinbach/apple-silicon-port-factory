// mingw's sal.h plus the few SAL annotations the VS 2022 17.9 STL uses that it lacks.
#pragma once
#include_next <sal.h>
#ifndef _Check_return_opt_
#define _Check_return_opt_
#endif
#ifndef _Deref_post_opt_valid_
#define _Deref_post_opt_valid_
#endif
