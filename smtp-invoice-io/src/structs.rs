#[derive(Serialize, Deserialize, Debug, Default)]
struct Impuesto {
    Codigo: String,
    CodigoTarifa: String,
    Tarifa: String,
    Monto: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct CodigoComercial {
    Tipo: String,
    Codigo: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Identificacion {
    Tipo: String,
    Numero: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Ubicacion {
    Provincia: String,
    Canton: String,
    Distrito: String,
    OtrasSenas: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Telefono {
    CodigoPais: String,
    NumTelefono: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CodigoTipoMoneda {
    CodigoMoneda: String,
    TipoCambio: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct OtroTexto {
    #[serde(rename = "$value", default)]
    Body: String,
    codigo: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Emisor {
    Nombre: String,
    Identificacion: Identificacion,
    #[serde(default)]
    NombreComercial: String,
    Ubicacion: Ubicacion,
    #[serde(default)]
    Telefono: Telefono,
    #[serde(default)]
    CorreoElectronico: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Receptor {
    Nombre: String,
    Identificacion: Identificacion,
    #[serde(default)]
    Telefono: Telefono,
    #[serde(default)]
    CorreoElectronico: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MensajeHacienda {
    Clave: String,
    NombreEmisor: String,
    TipoIdentificacionEmisor: String,
    NumeroCedulaEmisor: String,
    NombreReceptor: String,
    TipoIdentificacionReceptor: String,
    NumeroCedulaReceptor: String,
    Mensaje: String,
    #[serde(default)]
    DetalleMensaje: String,
    MontoTotalImpuesto: String,
    TotalFactura: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FacturaElectronica {
    Clave: String,
    CodigoActividad: String,
    NumeroConsecutivo: String,
    FechaEmision: String,
    Emisor: Emisor,
    Receptor: Receptor,
    CondicionVenta: String,
    MedioPago: String,
    DetalleServicio: DetalleServicio,
    ResumenFactura: ResumenFactura,
    #[serde(default)]
    Otros: Otros,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Otros {
    OtroTexto: Vec<OtroTexto>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResumenFactura {
    CodigoTipoMoneda: CodigoTipoMoneda,
    #[serde(default)]
    TotalServGravados: String,
    #[serde(default)]
    TotalServExentos: String,
    #[serde(default)]
    TotalServExonerado: String,
    #[serde(default)]
    TotalMercanciasGravadas: String,
    #[serde(default)]
    TotalMercanciasExentas: String,
    #[serde(default)]
    TotalMercExonerada: String,
    #[serde(default)]
    TotalGravado: String,
    #[serde(default)]
    TotalExento: String,
    #[serde(default)]
    TotalExonerado: String,
    #[serde(default)]
    TotalVenta: String,
    #[serde(default)]
    TotalDescuentos: String,
    #[serde(default)]
    TotalVentaNeta: String,
    #[serde(default)]
    TotalImpuesto: String,
    #[serde(default)]
    TotalIVADevuelto: String,
    #[serde(default)]
    TotalOtrosCargos: String,
    #[serde(default)]
    TotalComprobante: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DetalleServicio {
    LineaDetalle: Vec<LineaDetalle>,
}
#[derive(Serialize, Deserialize, Debug)]
struct LineaDetalle {
    NumeroLinea: String,
    #[serde(default)]
    CodigoComercial: CodigoComercial,
    Cantidad: String,
    UnidadMedida: String,
    Detalle: String,
    PrecioUnitario: String,
    MontoTotal: String,
    SubTotal: String,
    #[serde(default)]
    Impuesto: Impuesto,
    #[serde(default)]
    ImpuestoNeto: String,
    MontoTotalLinea: String,
}
