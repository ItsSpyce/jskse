#pragma once

#include <d3d11.h>

namespace graphics {
inline ID3D11Device* get_device() {
  return reinterpret_cast<ID3D11Device*>(RE::BSGraphics::Renderer::GetDevice());
}

inline ID3D11DeviceContext* get_context() {
  const auto renderer = RE::BSGraphics::Renderer::GetSingleton();
  return reinterpret_cast<ID3D11DeviceContext*>(renderer->data.context);
}

inline IDXGISwapChain* get_swap_chain() {
  const auto renderer = RE::BSGraphics::Renderer::GetSingleton();
  return reinterpret_cast<IDXGISwapChain*>(
      renderer->data.renderWindows[0].swapChain);
}

inline DXGI_SWAP_CHAIN_DESC get_swap_chain_desc() {
  DXGI_SWAP_CHAIN_DESC desc{};
  if (const auto result = get_swap_chain()->GetDesc(&desc); FAILED(result)) {
    throw std::runtime_error("Failed to get swap chain description");
  }
  return desc;
}
}  // namespace graphics