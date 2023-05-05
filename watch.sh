# cargo watch -q -c -x 'expand test'
# cargo watch -q -c -x 'expand test_string'
# cargo watch -q -c -x 'expand somerandomfunction'
# cargo watch -q -c -x 'expand parse'
RUSTFLAGS="-Z macro-backtrace" cargo +nightly watch -q -c -x 'expand --lib run'
# RUSTFLAGS="-Z macro-backtrace" cargo +nightly watch -q -c -x 'expand --lib jsx/signal'

# trunk serve
# trunk build
# python3 -m http.server
