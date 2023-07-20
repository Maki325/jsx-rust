# cargo watch -q -c -x 'expand test'
# cargo watch -q -c -x 'expand test_string'
# cargo watch -q -c -x 'expand somerandomfunction'
# cargo watch -q -c -x 'expand parse'
# RUSTFLAGS="-Z macro-backtrace" cargo +nightly watch -q -c -x 'expand --lib run'
# RUSTFLAGS="-Z macro-backtrace" cargo +nightly watch -q -c -x 'expand --lib example_const_read_signals'
RUSTFLAGS="-Z macro-backtrace" cargo +nightly watch -q -c -x 'expand --lib ExampleComponent'
# RUSTFLAGS="-Z macro-backtrace" cargo +nightly watch -q -c -x 'expand --lib example_counter'
# RUSTFLAGS="-Z macro-backtrace" cargo +nightly watch -q -c -x 'expand --lib cargo expand element::parse::get_name'
# RUSTFLAGS="-Z macro-backtrace" cargo +nightly watch -q -c -x 'expand --lib jsx/signal'

# trunk serve
# trunk build
# python3 -m http.server
