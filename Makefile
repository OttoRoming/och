LFS_VERSION := 13.1
LFS_ARCHIVE := LFS-BOOK-$(LFS_VERSION).tar.xz
LFS_URL := https://www.linuxfromscratch.org/lfs/downloads/stable-systemd/$(LFS_ARCHIVE)

target:
	mkdir -v target

target/$(LFS_ARCHIVE): target
	curl -o $@ $(LFS_URL)

lfs: target/$(LFS_ARCHIVE)
	mkdir -v lfs
	tar xvf target/$(LFS_ARCHIVE) -C lfs

clean:
	rm -rvf lfs target

.PHONY: clean
