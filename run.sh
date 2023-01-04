# cargo test
cargo run
# set -eou pipefail

# source .env/bin/activate
# pip3 install -U pip ipython maturin pytest pytest-benchmark
# python3 -m maturin develop --bindings pyo3 --release
# python3 -m IPython -c "
#     from IPython import get_ipython;
#     from rust2py import *;
#     get_ipython().run_line_magic('timeit', 'sum1(1,2)')
#     get_ipython().run_line_magic('timeit', 'sum2(1,2)')
#     get_ipython().run_line_magic('timeit', 'sum_i32_range(1,1000)')
#     get_ipython().run_line_magic('timeit', 'double(10)')
#     "
