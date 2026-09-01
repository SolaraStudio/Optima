use std::collections::VecDeque;
use std::time::Duration;

pub struct MediaStream {
    packets: VecDeque<MediaPacket>,
    current_pts: Duration,
    duration: Duration,
    is_eof: bool,
}

#[derive(Debug, Clone)]
pub struct MediaPacket {
    pub data: Vec<u8>,
    pub pts: u64,
    pub dts: u64,
    pub duration: u64,
    pub stream_index: usize,
    pub is_keyframe: bool,
}

impl MediaStream {
    pub fn new() -> Self {
        Self {
            packets: VecDeque::new(),
            current_pts: Duration::from_secs(0),
            duration: Duration::from_secs(0),
            is_eof: false,
        }
    }

    pub fn push_packet(&mut self, packet: MediaPacket) {
        self.packets.push_back(packet);
    }

    pub fn pop_packet(&mut self) -> Option<MediaPacket> {
        let packet = self.packets.pop_front();
        if let Some(ref p) = packet {
            self.current_pts = Duration::from_micros(p.pts);
        }
        packet
    }

    pub fn peek_packet(&self) -> Option<&MediaPacket> {
        self.packets.front()
    }

    pub fn is_empty(&self) -> bool {
        self.packets.is_empty()
    }

    pub fn len(&self) -> usize {
        self.packets.len()
    }

    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }

    pub fn get_duration(&self) -> Duration {
        self.duration
    }

    pub fn get_current_pts(&self) -> Duration {
        self.current_pts
    }

    pub fn set_eof(&mut self) {
        self.is_eof = true;
    }

    pub fn is_eof(&self) -> bool {
        self.is_eof
    }

    pub fn clear(&mut self) {
        self.packets.clear();
        self.current_pts = Duration::from_secs(0);
        self.is_eof = false;
    }
}

impl Default for MediaStream {
    fn default() -> Self {
        Self::new()
    }
}
