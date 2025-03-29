// SPDX-License-Identifier: BSD-3-Clause
#![warn(clippy::unwrap_used)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_snake_case)]

use m3u::{M3UEntry, M3UFile};

#[test]
fn testSingleLine()
{
	let line = "/foo/bar/file.mp3";
	let lines = M3UFile::fromStr(line);
	assert_eq!(lines.len(), 1);
	assert_eq!(lines[0], M3UEntry::AbsolutePath(line.into()))
}

#[test]
fn testSingleLineTrim()
{
	let line = "/foo/bar/file.mp3\t  ";
	let lines = M3UFile::fromStr(line);
	assert_eq!(lines.len(), 1);
	assert_eq!(lines[0], M3UEntry::AbsolutePath(line.trim().into()))
}

#[test]
fn testMultpleLines()
{
	let data = r#"
/foo/bar/file.mp3
file2.mp3
https://examample.com/file.mp3
"#;
	let lines = M3UFile::fromStr(data);
	assert_eq!(lines.len(), 3);
	assert_eq!(lines[0], M3UEntry::AbsolutePath("/foo/bar/file.mp3".into()));
	assert_eq!(lines[1], M3UEntry::RelativePath("file2.mp3".into()));
	assert_eq!(lines[2], M3UEntry::URL("https://examample.com/file.mp3".into()));
}

#[test]
fn testMultpleLinesIgnoreComments()
{
	let data = r#"
# test 123
/foo/bar/file.mp3
file2.mp3
# test 223
https://examample.com/file.mp3
"#;
	let lines = M3UFile::fromStr(data);
	assert_eq!(lines.len(), 3);
	assert_eq!(lines[0], M3UEntry::AbsolutePath("/foo/bar/file.mp3".into()));
	assert_eq!(lines[1], M3UEntry::RelativePath("file2.mp3".into()));
	assert_eq!(lines[2], M3UEntry::URL("https://examample.com/file.mp3".into()));
}


