package org.churchill.utils;

import org.apache.pdfbox.pdmodel.PDDocument;
import org.apache.pdfbox.printing.PDFPageable;

import javax.print.PrintService;
import javax.print.PrintServiceLookup;
import java.awt.print.PrinterJob;
import java.io.File;
import java.util.HashSet;



public class PrintHandler {
    HashSet<File> files;
    public PrintHandler(HashSet<File> files) {
        this.files = new HashSet<File>();
        this.files.addAll(files);
    }

    public void printAllFiles() {

        for (File file : files) {

            PDDocument document = null;

            try {
                document = PDDocument.load(file);
                PrinterJob job = PrinterJob.getPrinterJob();
                job.setPageable(new PDFPageable(document));
                PrintService service = PrintServiceLookup.lookupDefaultPrintService();
                job.setPrintService(service);
                job.print();
                System.out.printf("%s" , file.toString());
            } catch (Exception e) {

            }

        }

    }






}
