use super::*;

impl ServerInfo {
    pub fn is_connected(&mut self, peer_addr: SocketAddr) -> bool {
        self.connections.contains_key(&peer_addr)
    }

    pub fn get_connection(&self, peer_addr: SocketAddr) -> Result<&Connection, ErrorCode> {
        let con = self
            .connections
            .get(&peer_addr)
            .ok_or(ErrorCode::INVALID_COMMAND)?;
        Ok(con)
    }

    pub fn get_connection_mut(
        &mut self,
        peer_addr: SocketAddr,
    ) -> Result<&mut Connection, ErrorCode> {
        let con = self
            .connections
            .get_mut(&peer_addr)
            .ok_or(ErrorCode::INVALID_COMMAND)?;
        Ok(con)
    }

    pub(super) fn get_name_addr(&self, name: String) -> Result<&SocketAddr, ErrorCode> {
        let addr = self
            .name_to_addr
            .get(&name)
            .ok_or(ErrorCode::INVALID_ARGS)?;
        Ok(addr)
    }

    pub fn get_player_room(&self, peer_addr: SocketAddr) -> Result<&Room, ErrorCode> {
        let room_name = &self.get_player(peer_addr)?.location;

        let room = match self.world.rooms.get(room_name) {
            Some(room) => room,
            None => return Err(ErrorCode::ROOM_NOT_FOUND),
        };
        Ok(room)
    }
}
