// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include "ffi.h"
#include "fp.h"
#include "fpext.h"

#include <cstdint>
#include <stdexcept>

using namespace risc0;

extern "C" const char* risc0_circuit_string_ptr(risc0_string* str) {
  return str->str.c_str();
}

extern "C" void risc0_circuit_string_free(risc0_string* str) {
  delete str;
}

struct BridgeContext {
  void* ctx;
  Callback* callback;
};

void bridgeCallback(void* ctx,
                    const char* name,
                    const char* extra,
                    const Fp* args_ptr,
                    size_t args_len,
                    Fp* outs_ptr,
                    size_t outs_len) {
  BridgeContext* bridgeCtx = reinterpret_cast<BridgeContext*>(ctx);
  if (!bridgeCtx->callback(bridgeCtx->ctx, name, extra, args_ptr, args_len, outs_ptr, outs_len)) {
    throw std::runtime_error("Host callback failure");
  }
}

extern "C" uint32_t risc0_circuit_rv32im_step_compute_accum(risc0_error* err,
                                                            void* ctx,
                                                            Callback callback,
                                                            size_t steps,
                                                            size_t cycle,
                                                            Fp** args_ptr,
                                                            size_t /*args_len*/) {
  return ffi_wrap<uint32_t>(err, 0, [&] {
    BridgeContext bridgeCtx{ctx, callback};
    return circuit::rv32im::step_compute_accum(&bridgeCtx, bridgeCallback, steps, cycle, args_ptr)
        .asRaw();
  });
}

extern "C" uint32_t risc0_circuit_rv32im_step_verify_accum(risc0_error* err,
                                                           void* ctx,
                                                           Callback callback,
                                                           size_t steps,
                                                           size_t cycle,
                                                           Fp** args_ptr,
                                                           size_t /*args_len*/) {
  return ffi_wrap<uint32_t>(err, 0, [&] {
    BridgeContext bridgeCtx{ctx, callback};
    return circuit::rv32im::step_verify_accum(&bridgeCtx, bridgeCallback, steps, cycle, args_ptr)
        .asRaw();
  });
}

extern "C" uint32_t risc0_circuit_rv32im_step_exec(risc0_error* err,
                                                   void* ctx,
                                                   Callback callback,
                                                   size_t steps,
                                                   size_t cycle,
                                                   Fp** args_ptr,
                                                   size_t /*args_len*/) {
  return ffi_wrap<uint32_t>(err, 0, [&] {
    BridgeContext bridgeCtx{ctx, callback};
    return circuit::rv32im::step_exec(&bridgeCtx, bridgeCallback, steps, cycle, args_ptr).asRaw();
  });
}

extern "C" uint32_t risc0_circuit_rv32im_step_verify_bytes(risc0_error* err,
                                                           void* ctx,
                                                           Callback callback,
                                                           size_t steps,
                                                           size_t cycle,
                                                           Fp** args_ptr,
                                                           size_t /*args_len*/) {
  return ffi_wrap<uint32_t>(err, 0, [&] {
    BridgeContext bridgeCtx{ctx, callback};
    return circuit::rv32im::step_verify_bytes(&bridgeCtx, bridgeCallback, steps, cycle, args_ptr)
        .asRaw();
  });
}

extern "C" uint32_t risc0_circuit_rv32im_step_verify_mem(risc0_error* err,
                                                         void* ctx,
                                                         Callback callback,
                                                         size_t steps,
                                                         size_t cycle,
                                                         Fp** args_ptr,
                                                         size_t /*args_len*/) {
  return ffi_wrap<uint32_t>(err, 0, [&] {
    BridgeContext bridgeCtx{ctx, callback};
    return circuit::rv32im::step_verify_mem(&bridgeCtx, bridgeCallback, steps, cycle, args_ptr)
        .asRaw();
  });
}

#if defined(__clang__)
#pragma clang diagnostic ignored "-Wreturn-type-c-linkage"
#endif

extern "C" FpExt
risc0_circuit_rv32im_poly_fp(size_t cycle, size_t steps, FpExt* poly_mix, Fp** args) {
  return circuit::rv32im::poly_fp(cycle, steps, poly_mix, args);
}

constexpr FpExt FpExt::operator*(FpExt rhs) const {
  return FpExt();
  // // Rename the element arrays to something small for readability
  // #define a elems
  // #define b rhs.elems
  //   Fp c[2 * 4 + 1];
  //   for (size_t i = 0; i < 4 + 1; i++) {
  //     for (size_t j = 0; j < 4 + 1; j++) {
  //       c[i + j] += a[i] * b[j];
  //     }
  //   }
  //   return FpExt(c[0] - c[5] * 2, c[1] - c[6] * 2, c[2] - c[7] * 2, c[3] - c[8] * 2, c[4]);
  // #undef a
  // #undef b
}
