./target/release/nora-rust-engine \
--directory "/gppd/intuita/terraform-website" \
--pattern "**/pages/**/*.{ts,tsx}" \
--antipatterns "**/node_modules/*/**" \
--output-directory-path "/gppd/intuita/test"