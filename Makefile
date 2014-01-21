all: test

DIRS=$(shell ls -d src/set*/*)
.PHONY: $(DIRS)
$(DIRS):
	rustpkg build $(patsubst src/%,%,$@)
compile: $(DIRS)

i_DIRS=$(addprefix i_,$(DIRS))
.PHONY: $(i_DIRS)
$(i_DIRS):
	rustpkg install $(patsubst i_src/%,%,$@)
install: $(i_DIRS)

t_DIRS=$(addprefix t_,$(DIRS))
.PHONY: $(t_DIRS)
$(t_DIRS):
	rustpkg test $(patsubst t_src/%,%,$@)
test: $(t_DIRS)

clean:
	rustpkg clean
	$(RM) -r bin/ build/ lib/
