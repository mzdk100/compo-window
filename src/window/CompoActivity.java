package rust.compo;

import android.app.Activity;
import android.os.Bundle;

/**
 * CompoActivity - A singleton Activity manager for Rust Compo framework
 * 
 * This class provides a static method to access the current Activity instance
 * and ensures only one Activity instance exists at a time.
 */
public class CompoActivity extends Activity {
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        on_created();
    }
    
    @Override
    protected void onDestroy() {
        super.onDestroy();
        on_destroyed();
    }

    private native void on_created();
    private native void on_destroyed();
}