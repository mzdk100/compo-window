package rust.compo.android;

import android.os.Bundle;
import rust.compo.CompoActivity;

public class MainActivity extends CompoActivity {
    static {
        System.loadLibrary("cw_android");
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
    }
}