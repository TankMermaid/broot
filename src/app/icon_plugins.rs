use std::collections::hash_map::HashMap;
use crate::tree::TreeLineType;

pub trait IconPlugin
{
	fn get_icon( 
		&self,
		tree_line_type : &TreeLineType,

		// Use case: 
		// For files- use libmagic to get file type
		// For directories: get list of files to get dir type
		// Recommended to avoid for performance reasons. 
		full_path	   : &std::path::PathBuf,
		name           : &str,
		double_ext     : Option<&str>,
		ext            : Option<&str>,
	) -> char;
}

struct IconPluginVSCode{
	icon_name_to_icon_codepoint_map	 : HashMap<&'static str, u32>,
	file_name_to_icon_name_map	     : HashMap<&'static str, &'static str>,
	double_extension_to_icon_name_map: HashMap<&'static str, &'static str>,
	extension_to_icon_name_map       : HashMap<&'static str, &'static str>,
	default_icon_point               : u32,
}

impl IconPluginVSCode
{
	fn new() -> Self
	{
		let icon_name_to_icon_codepoint_map: HashMap<&'static str, u32> 
			= ( include!( "../../resources/icons/vscode/data/icon_name_to_icon_code_point_map.rs" ) ).iter().cloned().collect()
		;

		let double_extension_to_icon_name_map: HashMap<&'static str, &'static str>
			= ( include!( "../../resources/icons/vscode/data/double_extension_to_icon_name_map.rs" ) ).iter().cloned().collect()
		;

		let extension_to_icon_name_map       : HashMap<&'static str, &'static str>
			= ( include!( "../../resources/icons/vscode/data/extension_to_icon_name_map.rs" ) ).iter().cloned().collect()
		;
		
		let file_name_to_icon_name_map       : HashMap<&'static str, &'static str>
			= ( include!( "../../resources/icons/vscode/data/file_name_to_icon_name_map.rs" ) ).iter().cloned().collect()
		;

		let default_icon_point				  = *icon_name_to_icon_codepoint_map.get( "default_file" ).unwrap();
		Self{
			icon_name_to_icon_codepoint_map,
			file_name_to_icon_name_map,
			double_extension_to_icon_name_map,
			extension_to_icon_name_map,
			default_icon_point,
		}
	}

	fn handle_single_extension(
		&self,
		ext: Option<String>
	) -> &'static str
	{
		match ext 
		{
			None       => "default_file",
			Some( ref e )  => {
				match self.extension_to_icon_name_map.get( e as &str )
				{
					None => "default_file",
					Some( icon_name ) => icon_name,
				}
			}
		}
	}


	fn handle_file( 
		&self,
		name: &str,
		double_ext: Option<String>,
		ext: Option<String>,
	) -> &'static str
	{
		match self.file_name_to_icon_name_map.get( name )
		{
			Some( icon_name ) => icon_name,
			_ => self.handle_double_extension( double_ext, ext )
		}
	}
	
	fn handle_double_extension( 
		&self,
		double_ext: Option<String>,
		ext: Option<String>,
	) -> &'static str
	{
		match double_ext 
		{
			None       => self.handle_single_extension( ext ),
			Some( ref de ) => {
				match self.double_extension_to_icon_name_map.get( de as &str )
				{
					None => self.handle_single_extension( ext ),
					Some( icon_name ) => icon_name,
				}
			}
		}
	}
}

impl IconPlugin for IconPluginVSCode
{
	fn get_icon( 
		&self,
		tree_line_type: &TreeLineType,
		_full_path: &std::path::PathBuf,
		name: &str,
		double_ext: Option<&str>,
		ext: Option<&str>,
	) -> char
	{
		let icon_name = match tree_line_type
		{
			TreeLineType::Dir			=> "default_folder",
			TreeLineType::SymLink{ .. } => "file_type_kite", //bad but nothing better
			TreeLineType::File			=> self.handle_file( 
									      		&name.to_ascii_lowercase(),
									      		double_ext.map( |de| de.to_ascii_lowercase() ),
									      		ext.map( |e| e.to_ascii_lowercase() ),
									    	),
			TreeLineType::Pruning		=> "file_type_kite", //irrelevant
			_							=> "default_file",
		};

		let entry_icon = unsafe{ 
			std::char::from_u32_unchecked( 
				*self.icon_name_to_icon_codepoint_map
					.get( icon_name )
					.unwrap_or( &self.default_icon_point )
			)
		};

		entry_icon
	}
}

pub fn icon_plugin( icon_set: &str ) -> Option<Box<dyn IconPlugin + Send + Sync >> 
{
	match icon_set 
	{
		"vscode" => Some( Box::new( IconPluginVSCode::new() ) ),
		_ => None
	}
}
