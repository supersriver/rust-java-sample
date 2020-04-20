package javasample;

import com.sun.jna.Library;
import com.sun.jna.Native;
import com.sun.jna.Pointer;

public interface Rustffi extends Library {
    Rustffi instance = Native.load("../rust/target/release/rustffi", Rustffi.class);

    Pointer hello();
    void freemem(Pointer pointer);
}
