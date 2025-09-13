// src/main.rs
trait TaxStrategy {
    fn calculate(&self, amount: f64) -> f64;
}

// Strategy VAT (16%)
struct Vat;
impl TaxStrategy for Vat {
    fn calculate(&self, amount: f64) -> f64 {
        amount * 0.16
    }
}

// Strategy: Fix ISR (10%)
struct IncomeTax;
impl TaxStrategy for IncomeTax {
    fn calculate(&self, amount: f64) -> f64 {
        amount * 0.10
    }
}

// Strategy: Without taxes;
struct NoTax;
impl TaxStrategy for NoTax {
    fn calculate(&self, _amount: f64) -> f64 {
        0.0
    }
}

// Context : Invoice struct with a strategy
struct Invoice<'a> {
    subtotal: f64,
    strategy: Box<dyn TaxStrategy + 'a>,
}

impl<'a> Invoice<'a> {
    fn new(subtotal: f64, strategy: Box<dyn TaxStrategy + 'a>) -> Self {
        Self { subtotal, strategy }
    }

    fn total(&self) -> f64 {
        self.subtotal + self.strategy.calculate(self.subtotal)
    }
}

fn main() {
    let invoice_vat = Invoice::new(1000.0, Box::new(Vat));
    println!("Invoice with VAT: ${}", invoice_vat.total());

    let invoice_income_tax = Invoice::new(1000.0, Box::new(IncomeTax));
    println!("Invoice with ISR: ${}", invoice_income_tax.total());

    let invoice_no_tax = Invoice::new(1000.0, Box::new(NoTax));
    println!("Invoice without taxes: ${}", invoice_no_tax.total());
}
