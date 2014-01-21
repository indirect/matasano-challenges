DIRS=$(shell ls -d src/set*/*)
i_DIRS=$(addprefix i_,$(DIRS))
t_DIRS=$(addprefix t_,$(DIRS))

.PHONY: $(DIRS) $(i_DIRS) $(t_DIRS)

$(DIRS):
	rustpkg build $(patsubst src/%,%,$@)

$(i_DIRS):
	rustpkg install $(patsubst i_src/%,%,$@)

$(t_DIRS):
	rustpkg test $(patsubst t_src/%,%,$@)

compile: $(DIRS)
install: $(i_DIRS)
test: $(t_DIRS)

clean:
	rustpkg clean
	$(RM) -r bin/ build/ lib/
