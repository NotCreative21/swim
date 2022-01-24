extern crate x11rb;

use protocol::xproto;
use protocol::xproto::ConnectionExt;
use x11rb::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub enum IPC {
    ClientMessage,
    KillActiveClient,
    SwitchTag,
    BorderPixel,
    BorderWidth,
    BackgroundPixel,
    Last,
}

pub fn get_ipc_atoms<C>(conn: &C) -> Result<[xproto::Atom; IPC::Last as usize]>
where
    C: connection::Connection,
{
    Ok([
        conn.intern_atom(false, b"_SWIM_CLIENT_MESSAGE")?
            .reply()?
            .atom,
        conn.intern_atom(false, b"_SWIM_KILL_ACTIVE_CLIENT")?
            .reply()?
            .atom,
        conn.intern_atom(false, b"_SWIM_SWITCH_TAG")?.reply()?.atom,
        conn.intern_atom(false, b"_SWIM_BORDER_PIXEL")?
            .reply()?
            .atom,
        conn.intern_atom(false, b"_SWIM_BORDER_WIDTH")?
            .reply()?
            .atom,
        conn.intern_atom(false, b"_SWIM_BACKGROUND_PIXEL")?
            .reply()?
            .atom,
    ])
}
