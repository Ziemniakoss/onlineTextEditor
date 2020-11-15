use crate::models::ProjectFile;

/// Operates CRUD operations on files. Each IProjectsFilesRepository
/// implementation must operate only on one project
pub trait IProjectsFilesRepository{
	fn create(file: ProjectFile) -> Result<ProjectFile, ProjectFileCreationError>;

	fn update(file: ProjectFile) -> Result<ProjectFile, ProjectFileUpdateError>;

	fn delete(file: ProjectFile) -> Result<ProjectFile, ProjectFileDeleteError>;
}

pub enum ProjectFileCreationError{

}

pub enum ProjectFileUpdateError{

}

pub enum ProjectFileDeleteError{

}

