# Registro de Ventas en Solana (Solana Engineering Track - WayLearn)

Aplicación descentralizada (dApp) desplegada en **Solana Devnet** construida con **Anchor (Rust)**, **Codama** y **React + TypeScript**.

## Datos del Despliegue y Evidencia en Devnet

- **Autor:** Kaleb Guadalupe Hernández Torres (`@kaleb2108`)
- **Program ID (Devnet):** `E6aT8XpxPyqKtttRxjhgoVoaGX4icqbzDxbNic9BuwNo`
- **Explorer del Programa:** https://explorer.solana.com/address/E6aT8XpxPyqKtttRxjhgoVoaGX4icqbzDxbNic9BuwNo?cluster=devnet
- **Wallet de Autoridad (Phantom Devnet):** `3J936jjVMWdCo36imyytawpCf61PxTMYjeU5txMBooxP`
- **Transacción de Baja de Cliente (Cierre de PDA en Devnet):** https://explorer.solana.com/tx/2dKZAQjY88UDHRm84c8VZKNZ6umyqgApA4LWjGTtHziJKpB1er5Z9aULPSmoTr1Uj5TRT8ENDNb3WPtKufstkU3?cluster=devnet

## Mejoras de Lógica de Negocio Implementadas

1. **Baja de Cliente con Cierre de Cuenta PDA (`dar_baja_cliente`):**
   - Permite dar de baja a un cliente existente y cerrar su cuenta PDA (`close = owner`), recuperando el *rent* hacia la autoridad del negocio.
   - Valida estrictamente que el cliente no tenga ventas activas (`cliente.compras == 0`); de lo contrario, rechaza la transacción con el error personalizado `ClienteConVentasActivas` (`6002`).
2. **Validaciones de Integridad en el Contrato (`lib.rs`):**
   - **Montos mayores a cero (`MontoInvalido` - `6003`):** Impide registrar ventas o actualizar importes con valor `0`.
   - **Prevención de actualizaciones redundantes (`MontoSinCambios` - `6004`):** Rechaza modificaciones cuando el nuevo importe es idéntico al monto actual.
   - **Campos de texto obligatorios (`CampoVacio` - `6005`):** Evita crear registros, clientes o ventas con cadenas vacías o solo espacios.
3. **Integración Completa Frontend + Codama:**
   - Cliente TypeScript regenerado a partir del IDL desplegado en Devnet y nueva pestaña **Baja cliente** integrada en `src/components/Registro.tsx`.