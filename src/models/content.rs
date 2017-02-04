use ::Response;

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
    pub fn children(&self) -> Result<Vec<Self>, String> {
        if self.item.leaf {
            Ok(Vec::new())
        } else {
            let children_uri = self.item.resource_uri.as_str();
            self.client.get_absolute::<Vec<ContentMetadata>>(children_uri).map(|x| x.into())
        }
    }

    pub fn with_children(self) -> Result<Vec<Self>, String> {
        match self.children() {
            Ok(mut children) => {
                children.insert(0, self);
                Ok(children)
            },
            Err(x) => Err(x)
        }
    }

    pub fn descendants(&self) -> Result<Vec<Self>, String> {
        match self.children() {
            Ok(children) => {
                if children.is_empty() {
                    Ok(children)
                } else {
                    Ok(children.iter().flat_map(|child| child.clone().with_descendants().unwrap()).collect::<Vec<Self>>())
                }
            },
            Err(x) => Err(x)
        }
    }

    pub fn with_descendants(self) -> Result<Vec<Self>, String> {
        match self.descendants() {
            Ok(mut descendants) => {
                descendants.insert(0, self);
                Ok(descendants)
            },
            Err(x) => Err(x)
        }
    }

    pub fn leaves(&self) -> Result<Vec<Self>, String> {
        match self.descendants() {
            Ok(descendants) => Ok(descendants.iter().filter(|&d| d.item.leaf).map(|d| d.to_owned()).collect::<Vec<Self>>()),
            Err(x) => Err(x)
        }
    }

    /*pub fn with_leaves(self) -> impl Iterator<Item = Response<'a, Content>> {
        let self_leaf = if self.item.leaf { Some(self) } else { None };
        self.descendants().chain(self_leaf)
    }*/
}
