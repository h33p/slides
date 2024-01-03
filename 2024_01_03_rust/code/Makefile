CXX += -march=x86-64-v2 -g -msse4.2 -arch x86_64 --std=c++20 -ffast-math

RUSTFLAGS = -C target-feature=+sse4.2
RUSTARGS = --release --target x86_64-apple-darwin

all: race_condition.debug.out race_condition.out aliasing_base.debug.out aliasing_base.out aliasing.debug.out aliasing.out rust

race_condition.debug.out: race_condition.cpp
	$(CXX) -O0 race_condition.cpp -o $@
	$(CXX) -O0 race_condition.cpp -S -emit-llvm

race_condition.out: race_condition.cpp
	$(CXX) -Ofast race_condition.cpp -o $@
	$(CXX) -Ofast race_condition.cpp -S -emit-llvm

aliasing_base.debug.out: aliasing_base.cpp
	$(CXX) -O0 aliasing_base.cpp -o $@
	$(CXX) -O0 aliasing_base.cpp -S -emit-llvm

aliasing_base.out: aliasing_base.cpp
	$(CXX) -Ofast aliasing_base.cpp -o $@
	$(CXX) -Ofast aliasing_base.cpp -S -emit-llvm

aliasing.debug.out: aliasing.cpp
	$(CXX) -O0 aliasing.cpp -o $@
	$(CXX) -O0 aliasing.cpp -S -emit-llvm

aliasing.out: aliasing.cpp
	$(CXX) -Ofast aliasing.cpp -o $@
	$(CXX) -Ofast aliasing.cpp -S -emit-llvm

rust: FORCE
	cd rust \
		&& RUSTFLAGS="$(RUSTFLAGS)" cargo build --examples $(RUSTARGS) \
		&& RUSTFLAGS="$(RUSTFLAGS)" cargo rustc --example noalias $(RUSTARGS) -- --emit=llvm-ir

clean:
	rm -f *.ll *.out
	cd rust
	cd rust && cargo clean $(RUSTARGS)

FORCE:

.PHONY: all clean
