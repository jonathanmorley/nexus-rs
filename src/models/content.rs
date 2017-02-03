use ::Response;

use std::iter;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ContentMetadata {
    #[serde(rename="resourceURI")]
    pub resource_uri: String,
    #[serde(rename="relativePath")]
    pub relative_path: String,
    pub text: String,
    pub leaf: bool,
    #[serde(rename="lastModified")]
    pub last_modified: String,
    #[serde(rename="sizeOnDisk")]
    pub size_on_disk: i64,
}

impl<'a> Response<'a, ContentMetadata> {
    pub fn children(&self) -> Vec<Self> {
        if self.item.leaf {
            Vec::new()
        } else {
            let children_uri = self.item.resource_uri.as_str();
            self.client.get_absolute::<Vec<ContentMetadata>>(children_uri).unwrap().into()
        }
    }

    pub fn with_children(self) -> Vec<Self> {
        let mut children = self.children();
        children.insert(0, self);
        children
    }

    pub fn descendants(&self) -> Vec<Self> {
        let children = self.children();
        if children.is_empty() {
            children
        } else {
            children.iter().flat_map(|child| child.clone().with_descendants()).collect::<Vec<Self>>()
        }
    }

    pub fn with_descendants(self) -> Vec<Self> {
        let mut descendants = self.descendants();
        descendants.insert(0, self);
        descendants
    }

    pub fn leaves(&self) -> Vec<Self> {
        self.descendants().iter().filter(|&d| d.item.leaf).map(|d| d.to_owned()).collect::<Vec<Self>>()
    }

    /*pub fn with_leaves(self) -> impl Iterator<Item = Response<'a, Content>> {
        let self_leaf = if self.item.leaf { Some(self) } else { None };
        self.descendants().chain(self_leaf)
    }*/
}
