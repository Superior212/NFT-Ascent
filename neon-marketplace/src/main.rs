// Export ABI for the contract
#[cfg(feature = "export-abi")]
fn main() {
    neon_marketplace::print_abi("INeonMarketplace", "solidity");
}
