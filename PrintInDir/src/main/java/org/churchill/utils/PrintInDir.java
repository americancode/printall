package org.churchill.utils;

import java.io.File;

public class PrintInDir {
    public static void main(String[] args) {
        FileHandler fHandler = null;


        if (args.length > 0) {
            fHandler = new FileHandler(new File(args[0]));
        }else {
            fHandler = new FileHandler(new File(System.getProperty("user.dir")));
        }

        PrintHandler pHandler = new PrintHandler(fHandler.getAll());
        fHandler.printAll();
        pHandler.printAllFiles();
    }
}
