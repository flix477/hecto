use crate::core::posts::folder::Folder;
use crate::core::posts::Post;

#[derive(Clone, Debug)]
pub enum FolderEntry<P, F> {
    Post(P),
    Folder(F),
}

impl<P, F> FolderEntry<P, F> {
    pub fn as_ref(&self) -> FolderEntry<&P, &F> {
        match *self {
            FolderEntry::Post(ref post) => FolderEntry::Post(post),
            FolderEntry::Folder(ref folder) => FolderEntry::Folder(folder),
        }
    }

    pub fn as_mut_ref(&mut self) -> FolderEntry<&mut P, &mut F> {
        match *self {
            FolderEntry::Post(ref mut post) => FolderEntry::Post(post),
            FolderEntry::Folder(ref mut folder) => FolderEntry::Folder(folder),
        }
    }

    pub fn post(self) -> Option<P> {
        if let FolderEntry::Post(post) = self {
            Some(post)
        } else {
            None
        }
    }

    pub fn folder(self) -> Option<F> {
        if let FolderEntry::Folder(folder) = self {
            Some(folder)
        } else {
            None
        }
    }
}

impl FolderEntry<Post, Folder> {
    pub fn name(&self) -> String {
        match self {
            FolderEntry::Post(post) => post.name.clone(),
            FolderEntry::Folder(folder) => folder.name.clone(),
        }
    }
}

impl Into<FolderEntry<Post, Folder>> for Folder {
    fn into(self) -> FolderEntry<Post, Folder> {
        FolderEntry::Folder(self)
    }
}

impl<'a> Into<FolderEntry<&'a Post, &'a Folder>> for &'a Folder {
    fn into(self) -> FolderEntry<&'a Post, &'a Folder> {
        FolderEntry::Folder(self)
    }
}

impl<'a> Into<FolderEntry<&'a mut Post, &'a mut Folder>> for &'a mut Folder {
    fn into(self) -> FolderEntry<&'a mut Post, &'a mut Folder> {
        FolderEntry::Folder(self)
    }
}