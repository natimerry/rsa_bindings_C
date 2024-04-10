#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

extern "C" {

void generate_pub_keys();

void generate_priv_keys(uintptr_t bits);

} // extern "C"
