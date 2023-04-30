# cargo watch -q -c -x 'expand test'
# cargo watch -q -c -x 'expand test_string'
# cargo watch -q -c -x 'expand somerandomfunction'
# cargo watch -q -c -x 'expand parse'
RUSTFLAGS="-Z macro-backtrace" cargo +nightly watch -q -c -x 'expand --lib run'

# trunk serve
# trunk build
