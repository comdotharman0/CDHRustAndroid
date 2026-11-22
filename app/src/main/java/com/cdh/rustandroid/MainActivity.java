package com.cdh.rustandroid;

import android.app.Activity;
import android.os.Bundle;
import android.widget.Button;
import android.widget.TextView;

public class MainActivity extends Activity {
    static {
        System.loadLibrary("rust_engine");
    }

    private static native String stepEngine(long delta);

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        TextView statusText = findViewById(R.id.status_text);
        Button btnStep = findViewById(R.id.btn_step);

        btnStep.setOnClickListener(v -> {
            String result = stepEngine(1L);
            statusText.setText(result);
        });
    }
}
