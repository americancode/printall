package PrintInDirectory;

import org.docx4j.convert.out.pdf.viaXSLFO.PdfSettings;
import org.docx4j.fonts.IdentityPlusMapper;
import org.docx4j.fonts.Mapper;
import org.docx4j.fonts.PhysicalFonts;
import org.docx4j.openpackaging.packages.WordprocessingMLPackage;

import java.io.*;
import java.util.HashSet;
import java.util.List;

import static java.lang.System.out;

public class FileHandler {
    private File dir;
    private HashSet<File> files;




    public FileHandler(File path) {
        this.dir = path;
        this.files = new HashSet<File>();
    }


    public HashSet<File> getAll() {
        getFiles(this.dir);
        return files;
    }

    private void getFiles(File dir) {
        File fileArray[] = dir.listFiles();

        for(int i = 0; i < fileArray.length; i++) {
            if (fileArray[i].isDirectory()) {
                getFiles(fileArray[i].getAbsoluteFile());
            }else {
                if(fileArray[i].getAbsoluteFile().toString().matches(".*.pdf")) {
                    files.add(fileArray[i].getAbsoluteFile());
                }else if (fileArray[i].getAbsoluteFile().toString().matches(".*.docx")) {
                    files.add(docxToPDF(fileArray[i].getAbsoluteFile()));
                }
            }
        }

    }


    private File docxToPDF(File file) {
        disableErr();


        System.out.printf("Converting: %s\n", file.getName());
        InputStream is = null;
        File outputPdf = null;
        try {
            is = new FileInputStream(file);
            WordprocessingMLPackage wordMLPackage = WordprocessingMLPackage.load(is);
            List<?> sections = wordMLPackage.getDocumentModel().getSections();
            for (int i = 0; i < sections.size(); i++) {
                wordMLPackage.getDocumentModel().getSections().get(i).getPageDimensions();
            }

            Mapper fontMapper = new IdentityPlusMapper();
            org.docx4j.fonts.PhysicalFont font = PhysicalFonts.getPhysicalFonts().get(
                    "Comic Sans MS");//set your desired font
            fontMapper.getFontMappings().put("Algerian", font);
            wordMLPackage.setFontMapper(fontMapper);
            PdfSettings pdfSettings = new PdfSettings();
            org.docx4j.convert.out.pdf.PdfConversion conversion = new org.docx4j.convert.out.pdf.viaXSLFO.Conversion(wordMLPackage);
            outputPdf = new File(file.getParent() + "/" + file.getName() + ".pdf");
            OutputStream out = new FileOutputStream(outputPdf);
            conversion.output(out, pdfSettings);
            System.out.printf("Finished conversion of: %s\n", outputPdf.getName());
        } catch (Exception e) {
            e.printStackTrace();
        }
        return outputPdf;

    }


    private void disableErr(){
        System.setErr(new PrintStream(new OutputStream() {
            public void write(int b) {
            }
        }));
    }



    public void printAll() {
        for (File file : files) {
            out.printf("%s\n" , file);
        }
    }




}
