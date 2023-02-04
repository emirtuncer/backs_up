import hashlib
import shutil
import os
import re
import sys


def get_hash(filename):
    sha1 = hashlib.sha1()
    with open(filename, "rb") as f:
        while True:
            block = f.read(2**10)  # read the file in blocks of 1024 bytes
            if not block:
                break
            sha1.update(block)
    return sha1.hexdigest()


def copy_if_needed(src_file, dst_file):
    if not os.path.exists(dst_file):
        shutil.copy2(src_file, dst_file)
        return

    src_stat = os.stat(src_file)
    dst_stat = os.stat(dst_file)
    if src_stat.st_size != dst_stat.st_size or src_stat.st_mtime != dst_stat.st_mtime:
        if get_hash(src_file) != get_hash(dst_file):
            shutil.copy2(src_file, dst_file)


def should_ignore(name, ignore_patterns):
    for pattern in ignore_patterns:
        if pattern.match(name):
            return True
    return False


def load_ignore_patterns(ignore_file):
    ignore_patterns = []
    with open(ignore_file, "r") as f:
        for line in f:
            line = line.strip()
            if line and not line.startswith("#"):
                ignore_patterns.append(re.compile(line))
    return ignore_patterns


def copy_dir_tree(src_dir, dst_dir, ignore_patterns, total_files, copied_files):
    if not os.path.exists(dst_dir):
        os.makedirs(dst_dir)

    for item in os.listdir(src_dir):
        if should_ignore(item, ignore_patterns):
            continue

        s = os.path.join(src_dir, item)
        d = os.path.join(dst_dir, item)
        if os.path.isdir(s):
            copy_dir_tree(s, d, ignore_patterns, total_files, copied_files)
        else:
            copy_if_needed(s, d)
            copied_files[0] += 1
            sys.stdout.write(f"\rCopied {copied_files[0]}/{total_files} files")
            sys.stdout.flush()


def count_files(src_dir):
    count = 0
    for item in os.listdir(src_dir):
        s = os.path.join(src_dir, item)
        if os.path.isdir(s):
            count += count_files(s)
        else:
            count += 1
    return count


src_dir = "/Users/emirtuncer/Desktop/test"
dst_dir = "./test"
ignore_file = "./ignore.txt"
ignore_patterns = load_ignore_patterns(ignore_file)


total_files = count_files(src_dir)
copied_files = [0]


print(f"Copying {total_files} files...")
copy_dir_tree(src_dir, dst_dir, ignore_patterns, total_files, copied_files)
print(f"\nFinished copying {copied_files[0]} files")
