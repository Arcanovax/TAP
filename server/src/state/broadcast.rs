use super::*;
use crate::structures::enums::error::ErrorCode;

impl ServerInfo {
    pub fn get_global_receivers(&mut self, peer_addr: SocketAddr) -> Vec<&Connection> {
        let mut receivers = Vec::new();

        for con in self.connections.values() {
            if con.addr != peer_addr {
                receivers.push(con);
            }
        }
        receivers
    }

    pub fn get_group_receivers(
        &mut self,
        peer_addr: SocketAddr,
    ) -> Result<Vec<&Connection>, ErrorCode> {
        let sender_con = self.get_connection(peer_addr)?;
        let group_id = sender_con.player.group_id.ok_or(ErrorCode::NOT_IN_GROUP)?;
        let receivers = self
            .groups
            .get(&group_id)
            .ok_or(ErrorCode::INVALID_COMMAND)?
            .players
            .iter()
            .filter_map(|addr| self.connections.get(addr))
            .filter(|con| con.addr != peer_addr)
            .collect();
        Ok(receivers)
    }

    pub fn get_room_receivers(&self, peer_addr: SocketAddr) -> Result<Vec<&Connection>, ErrorCode> {
        let room = &self.get_player(peer_addr)?.location;
        let mut receivers: Vec<&Connection> = Vec::new();
        for con in self.connections.values() {
            if con.addr != peer_addr && con.player.location == *room {
                receivers.push(con);
            }
        }
        Ok(receivers)
    }
}
