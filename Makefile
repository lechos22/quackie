CFLAGS=-Wall -Wextra -Werror -O3 -g
TMPDIR=/tmp/quackie
DESTDIR:=/usr/local
NAME=quackie
VERSION=0.1.0

all: dist/bin/quackie

run: dist/bin/quackie
	./dist/bin/quackie

install: dist/bin/quackie
    install -Dm755 dist/bin/quackie $(DESTDIR)/bin/quackie

uninstall:
	@rm $(DESTDIR)/bin/quackie

package: $(NAME)-$(VERSION).tar.gz $(NAME)-$(VERSION).deb

$(NAME)-$(VERSION).tar.gz: all
	@mkdir -p $(TMPDIR)/tar
	@cp -r dist/* $(TMPDIR)/tar
	@find $(TMPDIR)/tar -type f -executable -exec strip {} \;
	@tar -czf $@ -C $(TMPDIR)/tar .

$(NAME)-$(VERSION).deb: all
	@mkdir -p $(TMPDIR)/deb/DEBIAN
	@cp -r dist/* $(TMPDIR)/deb/
	@find $(TMPDIR)/deb -type f -executable -exec strip {} \;
	@echo "Package: quackie" > $(TMPDIR)/deb/DEBIAN/control
	@echo "Version: $(VERSION)" >> $(TMPDIR)/deb/DEBIAN/control
	@echo "Section: base" >> $(TMPDIR)/deb/DEBIAN/control
	@echo "Priority: optional" >> $(TMPDIR)/deb/DEBIAN/control
	@echo "Architecture: all" >> $(TMPDIR)/deb/DEBIAN/control
	@echo "Depends: " >> $(TMPDIR)/deb/DEBIAN/control
	@echo "Maintainer: lechos22" >> $(TMPDIR)/deb/DEBIAN/control
	@echo "Description: Quackie" >> $(TMPDIR)/deb/DEBIAN/control
	@dpkg-deb --build $(TMPDIR)/deb $@

OBJECTS = $(TMPDIR)/main.o

clean:
	@rm -rf $(TMPDIR) dist $(NAME)-$(VERSION).tar.gz $(NAME)-$(VERSION).deb

$(TMPDIR)/%.o: src/%.c $(TMPDIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(TMPDIR):
	@mkdir -p $(TMPDIR)

dist/bin/quackie: $(OBJECTS) dist/bin
	$(CC) -o $@ $(OBJECTS) $(CFLAGS) -lncurses -lm

dist/bin:
	@mkdir -p dist/bin

