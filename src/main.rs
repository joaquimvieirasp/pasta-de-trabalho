use xlsxwriter::{Workbook, Worksheet};

fn main() {
    // Criar uma pasta de trabalho
    let mut workbook = Workbook::new("saida.xlsx");

    // Criar uma planilha
    let mut worksheet = workbook.add_worksheet("Dados");

    // Escrever dados na planilha
    worksheet.write_string(0, 0, "Entrada");
    worksheet.write_string(0, 1, "Saída");
    worksheet.write_string(0, 2, "Total de horas trabalhadas");

    // Formatar as células
    worksheet.set_column_width(0, 15);
    worksheet.set_column_width(1, 15);
    worksheet.set_column_width(2, 25);

    // Salvar a pasta de trabalho
    workbook.save().unwrap();
}