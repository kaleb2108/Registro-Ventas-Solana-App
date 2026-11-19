use anchor_lang::prelude::*;

declare_id!("E6aT8XpxPyqKtttRxjhgoVoaGX4icqbzDxbNic9BuwNo");

#[program]
pub mod registro {
    use super::*;

    /////////////////////////// Crear Registro ///////////////////////////
    pub fn crear_registro(context: Context<NuevoRegistro>, nombre_negocio: String) -> Result<()> {
        require!(
            !nombre_negocio.trim().is_empty(),
            Errores::CampoVacio
        );

        context.accounts.registro.set_inner(Registro {
            owner: context.accounts.owner.key(),
            nombre_negocio: nombre_negocio.clone(),
            clientes: Vec::new(),
            id_counter: 0,
        });

        msg!("Registro: {} creado exitosamente!", nombre_negocio);

        Ok(())
    }

    /////////////////////////// Nuevo Cliente ///////////////////////////
    pub fn crear_cliente(context: Context<NuevoCliente>, nombre: String) -> Result<()> {
        require!(!nombre.trim().is_empty(), Errores::CampoVacio);

        let cliente_key = context.accounts.cliente.key();
        let registro = &mut context.accounts.registro;

        let id_cliente = registro.id_counter + 1;
        registro.id_counter = id_cliente;
        registro.clientes.push(cliente_key);

        context.accounts.cliente.set_inner(Cliente {
            id_cliente,
            nombre: nombre.clone(),
            compras: 0,
            historial: Vec::new(),
        });

        msg!("Cliente {} creado con id: {}", nombre, id_cliente);

        Ok(())
    }

    /////////////////////////// Dar de Baja Cliente (Día 5) ///////////////////////////
    pub fn dar_baja_cliente(
        context: Context<EliminarCliente>,
        id_cliente: u16,
    ) -> Result<()> {
        let cliente_key = context.accounts.cliente.key();
        let cliente = &context.accounts.cliente;

        // Regla de negocio: no se puede dar de baja si aun tiene ventas activas
        require!(
            cliente.historial.is_empty(),
            Errores::ClienteConVentasActivas
        );

        let registro = &mut context.accounts.registro;
        registro.clientes.retain(|&k| k != cliente_key);

        msg!("Cliente con id: {} dado de baja correctamente", id_cliente);

        Ok(())
    }

    /////////////////////////// Registrar Venta ///////////////////////////
    pub fn registrar_venta(
        context: Context<NuevaVenta>,
        _id_cliente: u16,
        sku: String,
        descripcion: String,
        monto: u32,
    ) -> Result<()> {
        // Mejora funcional: validaciones de negocio en Rust
        require!(!sku.trim().is_empty(), Errores::CampoVacio);
        require!(!descripcion.trim().is_empty(), Errores::CampoVacio);
        require!(monto > 0, Errores::MontoInvalido);

        context.accounts.venta.set_inner(Venta {
            sku,
            descripcion,
            monto,
        });

        let venta_key = context.accounts.venta.key();
        let cliente = &mut context.accounts.cliente;

        cliente.historial.push(venta_key);
        cliente.compras += 1;

        msg!(
            "Venta registrada para el cliente con id: {}",
            cliente.id_cliente
        );

        Ok(())
    }

    /////////////////////////// Hacer Devolucion ///////////////////////////
    pub fn hacer_devolucion(
        context: Context<EliminarVenta>,
        _id_cliente: u16,
        sku: String,
    ) -> Result<()> {
        let venta_key = context.accounts.venta.key();
        let cliente = &mut context.accounts.cliente;

        require!(cliente.historial.contains(&venta_key), Errores::NoSKU);

        cliente.historial.retain(|&k| k != venta_key);
        cliente.compras -= 1;

        msg!(
            "Venta con sku: {}, devuelta por el cliente con id: {}",
            sku,
            cliente.id_cliente
        );

        Ok(())
    }

    /////////////////////////// Cambiar Monto ///////////////////////////
    pub fn cambiar_monto(
        context: Context<ModificarVenta>,
        _id_cliente: u16,
        sku: String,
        nuevo_monto: u32,
    ) -> Result<()> {
        // Mejora funcional: rechazar monto 0 o igual al actual
        require!(nuevo_monto > 0, Errores::MontoInvalido);

        let venta = &mut context.accounts.venta;
        require!(venta.monto != nuevo_monto, Errores::MontoSinCambios);

        let viejo_monto = venta.monto;
        venta.monto = nuevo_monto;

        msg!(
            "El monto de la venta con sku: {}, paso de: {}, a: {}",
            sku,
            viejo_monto,
            nuevo_monto
        );

        Ok(())
    }
}

/////////////////////////// Codigos de Error ///////////////////////////
#[error_code]
pub enum Errores {
    #[msg("Error, no eres el owner del registro")]
    NoOwner,
    #[msg("Error, la venta no esta en el historial del cliente")]
    NoSKU,
    #[msg("Error, no se puede dar de baja a un cliente con ventas activas")]
    ClienteConVentasActivas,
    #[msg("Error, el monto debe ser mayor a cero")]
    MontoInvalido,
    #[msg("Error, el nuevo monto es igual al monto actual")]
    MontoSinCambios,
    #[msg("Error, los campos de texto no pueden estar vacios")]
    CampoVacio,
}

/////////////////////////// CUENTAS ///////////////////////////
#[account]
#[derive(InitSpace)]
pub struct Registro {
    pub owner: Pubkey,
    #[max_len(30)]
    nombre_negocio: String,
    #[max_len(10)]
    clientes: Vec<Pubkey>,
    id_counter: u16,
}

#[account]
#[derive(InitSpace, PartialEq)]
pub struct Cliente {
    id_cliente: u16,
    #[max_len(30)]
    nombre: String,
    compras: u16,
    #[max_len(10)]
    historial: Vec<Pubkey>,
}

#[account]
#[derive(InitSpace, PartialEq)]
pub struct Venta {
    #[max_len(5)]
    sku: String,
    #[max_len(50)]
    descripcion: String,
    monto: u32,
}

/////////////////////////// CONTEXTOS ///////////////////////////
#[derive(Accounts)]
#[instruction(nombre_negocio: String)]
pub struct NuevoRegistro<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Registro::INIT_SPACE + 8,
        seeds = [b"registro", nombre_negocio.as_bytes(), owner.key().as_ref()],
        bump
    )]
    pub registro: Account<'info, Registro>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevoCliente<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut, has_one = owner @ Errores::NoOwner)]
    pub registro: Account<'info, Registro>,

    #[account(
        init,
        payer = owner,
        space = Cliente::INIT_SPACE + 8,
        seeds = [b"cliente", registro.key().as_ref(), (registro.id_counter + 1).to_le_bytes().as_ref()],
        bump
    )]
    pub cliente: Account<'info, Cliente>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id_cliente: u16)]
pub struct EliminarCliente<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut, has_one = owner @ Errores::NoOwner)]
    pub registro: Account<'info, Registro>,

    #[account(
        mut,
        close = owner,
        seeds = [b"cliente", registro.key().as_ref(), id_cliente.to_le_bytes().as_ref()],
        bump
    )]
    pub cliente: Account<'info, Cliente>,
}

#[derive(Accounts)]
#[instruction(id_cliente: u16, sku: String)]
pub struct NuevaVenta<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(has_one = owner @ Errores::NoOwner)]
    pub registro: Account<'info, Registro>,

    #[account(
        mut,
        seeds = [b"cliente", registro.key().as_ref(), id_cliente.to_le_bytes().as_ref()],
        bump
    )]
    pub cliente: Account<'info, Cliente>,

    #[account(
        init,
        payer = owner,
        space = Venta::INIT_SPACE + 8,
        seeds = [b"venta", cliente.key().as_ref(), sku.as_bytes()],
        bump
    )]
    pub venta: Account<'info, Venta>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id_cliente: u16, sku: String)]
pub struct ModificarVenta<'info> {
    pub owner: Signer<'info>,

    #[account(has_one = owner @ Errores::NoOwner)]
    pub registro: Account<'info, Registro>,

    #[account(
        seeds = [b"cliente", registro.key().as_ref(), id_cliente.to_le_bytes().as_ref()],
        bump
    )]
    pub cliente: Account<'info, Cliente>,

    #[account(
        mut,
        seeds = [b"venta", cliente.key().as_ref(), sku.as_bytes()],
        bump
    )]
    pub venta: Account<'info, Venta>,
}

#[derive(Accounts)]
#[instruction(id_cliente: u16, sku: String)]
pub struct EliminarVenta<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(has_one = owner @ Errores::NoOwner)]
    pub registro: Account<'info, Registro>,

    #[account(
        mut,
        seeds = [b"cliente", registro.key().as_ref(), id_cliente.to_le_bytes().as_ref()],
        bump
    )]
    pub cliente: Account<'info, Cliente>,

    #[account(
        mut,
        close = owner,
        seeds = [b"venta", cliente.key().as_ref(), sku.as_bytes()],
        bump
    )]
    pub venta: Account<'info, Venta>,
}