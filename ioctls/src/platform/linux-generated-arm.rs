// Initially generated by process_ioctls.py for x86_64, then the evdev ioctls were transcluded.
ioctl!(read eviocgeffects with b'E', 0x84; ::std::os::raw::c_int);
ioctl!(read eviocgid with b'E', 0x02; /*struct*/ input_id);
ioctl!(read eviocgkeycode with b'E', 0x04; [::libc::c_uint; 2]);
// ioctl!(read eviocgkeycode_v2 with b'E', 0x04; /*struct*/ input_keymap_entry);
ioctl!(read eviocgrep with b'E', 0x03; [::libc::c_uint; 2]);
ioctl!(read eviocgversion with b'E', 0x01; ::std::os::raw::c_int);
ioctl!(write eviocrmff with b'E', 0x81; ::std::os::raw::c_int);
// TODO #define EVIOCSFF _IOC ( _IOC_WRITE , 'E' , 0x80 , sizeof ( struct ff_effect ) )
ioctl!(write eviocskeycode with b'E', 0x04; [::libc::c_uint; 2]);
// ioctl!(write eviocskeycode_v2 with b'E', 0x04; /*struct*/ input_keymap_entry);
ioctl!(write eviocsrep with b'E', 0x03; [::libc::c_uint; 2]);
