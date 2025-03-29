// SPDX-License-Identifier: BSD-3-Clause
#![warn(clippy::unwrap_used)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_snake_case)]

/// Location of a media file
#[derive(Debug, PartialEq)]
pub enum M3UEntry
{
	/// an absolute local pathname; e.g., C:\My Music\Heavysets.mp3
	AbsolutePath(String),
	/// a local pathname relative to the M3U file location; e.g. Heavysets.mp3
	RelativePath(String),
	/// a http/https url
	URL(String)
}

impl M3UEntry
{
	fn fromLine(line: &str) -> Self
	{
		// TODO: windows absolute paths
		if line.starts_with('/')
		{
			return Self::AbsolutePath(line.into())
		}

		if line.starts_with("http://") || line.starts_with("https://")
		{
			return Self::URL(line.into())
		}

		// assume that all other files are relative paths
		Self::RelativePath(line.into())
	}
}

pub struct M3UFile {}

impl M3UFile
{
	pub fn fromStr(data: &str) -> Vec<M3UEntry>
	{
		data
			.lines()
			.map(|line| line.trim())
			.filter(|line| !line.is_empty() && !line.starts_with("#"))
			.map(|line| M3UEntry::fromLine(line))
			.collect()
	}
}
