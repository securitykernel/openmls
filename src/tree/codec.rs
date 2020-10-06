use crate::tree::{astree::*, node::*, *};

impl Codec for NodeType {
    fn encode(&self, buffer: &mut Vec<u8>) -> Result<(), CodecError> {
        (*self as u8).encode(buffer)?;
        Ok(())
    }
    // fn decode(cursor: &mut Cursor) -> Result<Self, CodecError> {
    //     Ok(NodeType::from(u8::decode(cursor)?))
    // }
}

impl Codec for Node {
    fn encode(&self, buffer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.node_type.encode(buffer)?;
        self.key_package.encode(buffer)?;
        self.node.encode(buffer)?;
        Ok(())
    }
    // fn decode(cursor: &mut Cursor) -> Result<Self, CodecError> {
    //     let node_type = NodeType::decode(cursor)?;
    //     let key_package = Option::<KeyPackage>::decode(cursor)?;
    //     let node = Option::<ParentNode>::decode(cursor)?;
    //     Ok(Node {
    //         node_type,
    //         key_package,
    //         node,
    //     })
    // }
}

impl Codec for PathKeypairs {
    fn encode(&self, buffer: &mut Vec<u8>) -> Result<(), CodecError> {
        encode_vec(VecSize::VecU32, buffer, &self.keypairs)?;
        Ok(())
    }
    // fn decode(cursor: &mut Cursor) -> Result<Self, CodecError> {
    //     let keypairs = decode_vec(VecSize::VecU32, cursor)?;
    //     Ok(PathKeypairs { keypairs })
    // }
}

impl Codec for RatchetTree {
    fn encode(&self, buffer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.ciphersuite.encode(buffer)?;
        encode_vec(VecSize::VecU32, buffer, &self.nodes)?;
        self.path_keypairs.encode(buffer)?;
        self.own_node_index.as_u32().encode(buffer)?;
        Ok(())
    }
    // fn decode(cursor: &mut Cursor) -> Result<RatchetTree, CodecError> {
    //     let ciphersuite = Ciphersuite::decode(cursor)?;
    //     let nodes = decode_vec(VecSize::VecU32, cursor)?;
    //     let own_leaf = OwnLeaf::decode(cursor)?;
    //     Ok(RatchetTree {
    //         ciphersuite,
    //         nodes,
    //         own_leaf,
    //     })
    // }
}

impl Codec for UpdatePathNode {
    fn encode(&self, buffer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.public_key.encode(buffer)?;
        encode_vec(VecSize::VecU32, buffer, &self.encrypted_path_secret)?;
        Ok(())
    }
    // fn decode(cursor: &mut Cursor) -> Result<Self, CodecError> {
    //     let public_key = HPKEPublicKey::decode(cursor)?;
    //     let encrypted_path_secret = decode_vec(VecSize::VecU32, cursor)?;
    //     Ok(UpdatePathNode {
    //         public_key,
    //         encrypted_path_secret,
    //     })
    // }
}

impl Codec for UpdatePath {
    fn encode(&self, buffer: &mut Vec<u8>) -> Result<(), CodecError> {
        self.leaf_key_package.encode(buffer)?;
        encode_vec(VecSize::VecU16, buffer, &self.nodes)?;
        Ok(())
    }
    // fn decode(cursor: &mut Cursor) -> Result<Self, CodecError> {
    //     let leaf_key_package = KeyPackage::decode(cursor)?;
    //     let nodes = decode_vec(VecSize::VecU16, cursor)?;
    //     Ok(UpdatePath {
    //         leaf_key_package,
    //         nodes,
    //     })
    // }
}

// ASTree Codecs

impl Codec for ASTreeNode {
    fn encode(&self, buffer: &mut Vec<u8>) -> Result<(), CodecError> {
        encode_vec(VecSize::VecU8, buffer, &self.secret)?;
        Ok(())
    }
    // fn decode(cursor: &mut Cursor) -> Result<Self, CodecError> {
    //     let secret = decode_vec(VecSize::VecU8, cursor)?;
    //     Ok(ASTreeNode { secret })
    // }
}
