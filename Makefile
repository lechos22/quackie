CFLAGS=-Wall -Wextra -Werror -O3
TMPDIR=/tmp/quackie

all: dist/bin/quackie

run: dist/bin/quackie
	./dist/bin/quackie

install: dist/bin/quackie
	@cp dist/bin/quackie /usr/local/bin/quackie

uninstall:
	@rm /usr/local/bin/quackie

package: quackie.tar.gz quackie.deb

quackie.tar.gz: dist/bin/quackie
	@mkdir -p $(TMPDIR)/tar
	@cp -r dist/* $(TMPDIR)/tar
	@find $(TMPDIR)/tar -type f -executable -exec strip {} \;
	@tar -czf quackie.tar.gz -C $(TMPDIR)/tar .

quackie.deb: quackie.tar.gz
	@mkdir -p $(TMPDIR)/deb/DEBIAN
	@cp -r dist/* $(TMPDIR)/deb/
	@find $(TMPDIR)/deb -type f -executable -exec strip {} \;
	@echo "Package: quackie" > $(TMPDIR)/deb/DEBIAN/control
	@echo "Version: 0.1" >> $(TMPDIR)/deb/DEBIAN/control
	@echo "Section: base" >> $(TMPDIR)/deb/DEBIAN/control
	@echo "Priority: optional" >> $(TMPDIR)/deb/DEBIAN/control
	@echo "Architecture: all" >> $(TMPDIR)/deb/DEBIAN/control
	@echo "Depends: " >> $(TMPDIR)/deb/DEBIAN/control
	@echo "Maintainer: lechos22" >> $(TMPDIR)/deb/DEBIAN/control
	@echo "Description: Quackie" >> $(TMPDIR)/deb/DEBIAN/control
	@dpkg-deb --build $(TMPDIR)/deb quackie.deb

OBJECTS = $(TMPDIR)/main.o

clean:
	@rm -rf $(TMPDIR) dist

$(TMPDIR)/%.o: src/%.c $(TMPDIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(TMPDIR):
	@mkdir -p $(TMPDIR)

dist/bin/quackie: $(OBJECTS) dist/bin
	$(CC) -o $@ $(OBJECTS) $(CFLAGS) -lncurses -lm

dist/bin:
	@mkdir -p dist/bin

