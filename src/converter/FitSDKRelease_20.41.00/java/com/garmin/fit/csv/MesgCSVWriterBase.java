package com.garmin.fit.csv;

import com.garmin.fit.FieldBase;
import com.garmin.fit.Fit;

public class MesgCSVWriterBase {
   protected CSVWriter csv;
   protected boolean hideUnknownData = false;
   private boolean showInvalidsAsEmptyCells = false;
   private boolean printByteAsHex = false;

   public MesgCSVWriterBase(String fileName) {
      this.csv = new CSVWriter(fileName);
   }

   public void close() {
      csv.close();
   }

   public void showInvalidsAsEmptyCells() {
      showInvalidsAsEmptyCells = true;
   }

   public void hideUnknownData() {
      hideUnknownData = true;
   }

   public void setPrintByteAsHex(boolean value){
      printByteAsHex = value;
   }

   protected String getValueString(FieldBase field, int subFieldIndex) {
      Object value = field.getValue(0, subFieldIndex);
      String out;

      if(value == null || (showInvalidsAsEmptyCells && value.equals(Fit.baseTypeInvalidMap.get(field.getType(subFieldIndex))))) {
         out = "";
      }
      else{
         if(printByteAsHex && field.getType(subFieldIndex) == Fit.BASE_TYPE_BYTE) {
            out = String.format("%02x", value);
         }
         else {
            out = value.toString();
         }
      }

      for (int fieldElement = 1; fieldElement < field.getNumValues(); fieldElement++) {
         out += "|";

         value = field.getValue(fieldElement, subFieldIndex);

         if (value != null) {
            if (printByteAsHex && field.getType(subFieldIndex) == Fit.BASE_TYPE_BYTE) {
               out += String.format("%02x", value);
            }
            else {
               out += value.toString();
            }
         }
      }

      // Escapes embedded commas, double quotes, and newline characters
      out = out.replaceAll("\"", "\"\"");
      out = "\"" + out + "\"";
      return out;
   }
}
