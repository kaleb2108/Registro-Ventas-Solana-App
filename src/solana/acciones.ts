import { address, createNoopSigner, type Address } from '@solana/kit';
import type { Connection } from '@solana/web3.js';
import {
  getCrearRegistroInstruction, getCrearClienteInstruction, getDarBajaClienteInstruction,
  getRegistrarVentaInstruction, getCambiarMontoInstruction, getHacerDevolucionInstruction,
} from '../../clients/js/src/generated';
import { cadena, pdaCliente, pdaRegistro } from './pdas';
import { siguienteId } from './lecturas';
import { enviar, type WalletFirma, type OpcionesEnvio } from './tx';
import { comprobarConfiguracion } from './seguridad';
import { negocioValido, nombreValido, skuValido, descripcionValida, idValido } from './validaciones';

export function direccionDe(wallet: Pick<WalletFirma, 'publicKey'>): Address {
  if (!wallet.publicKey) throw new Error('Conecta tu wallet primero.');
  return address(wallet.publicKey.toBase58());
}

function firmante(wallet: WalletFirma) {
  comprobarConfiguracion();
  return createNoopSigner(direccionDe(wallet));
}

export async function crearRegistro(wallet: WalletFirma, connection: Connection, nombreNegocio: string, opciones?: OpcionesEnvio): Promise<string> {
  nombreNegocio = negocioValido(nombreNegocio);
  const owner = firmante(wallet);
  const registro = pdaRegistro(nombreNegocio, owner.address);
  const ix = getCrearRegistroInstruction({ owner, registro, nombreNegocio });
  return enviar(ix, wallet, connection, opciones);
}

export async function crearCliente(wallet: WalletFirma, connection: Connection, nombreNegocio: string, nombre: string, opciones?: OpcionesEnvio): Promise<string> {
  nombreNegocio = negocioValido(nombreNegocio);
  nombre = nombreValido(nombre);
  const owner = firmante(wallet);
  const registro = pdaRegistro(nombreNegocio, owner.address);
  const id = await siguienteId(nombreNegocio, owner.address);
  const cliente = pdaCliente(registro, id);
  const ix = getCrearClienteInstruction({ owner, registro, cliente, nombre });
  return enviar(ix, wallet, connection, opciones);
}

export async function darBajaCliente(wallet: WalletFirma, connection: Connection, nombreNegocio: string, idCliente: number, opciones?: OpcionesEnvio): Promise<string> {
  nombreNegocio = negocioValido(nombreNegocio);
  idCliente = idValido(idCliente);
  const owner = firmante(wallet);
  const registro = pdaRegistro(nombreNegocio, owner.address);
  const cliente = pdaCliente(registro, idCliente);
  const ix = getDarBajaClienteInstruction({ owner, registro, cliente, idCliente });
  return enviar(ix, wallet, connection, opciones);
}

export async function registrarVenta(wallet: WalletFirma, connection: Connection, nombreNegocio: string, idCliente: number, sku: string, descripcion: string, monto: number, opciones?: OpcionesEnvio): Promise<string> {
  nombreNegocio = negocioValido(nombreNegocio);
  idCliente = idValido(idCliente);
  sku = skuValido(sku);
  descripcion = descripcionValida(descripcion);
  if (!Number.isInteger(monto) || monto < 0 || monto > 4294967295) throw new Error('Ingresa un monto entero válido.');
  const owner = firmante(wallet);
  const { registro, cliente, venta } = cadena(nombreNegocio, owner.address, idCliente, sku);
  const ix = getRegistrarVentaInstruction({ owner, registro, cliente, venta, idCliente, sku, descripcion, monto });
  return enviar(ix, wallet, connection, opciones);
}

export async function cambiarMonto(wallet: WalletFirma, connection: Connection, nombreNegocio: string, idCliente: number, sku: string, nuevoMonto: number, opciones?: OpcionesEnvio): Promise<string> {
  nombreNegocio = negocioValido(nombreNegocio);
  idCliente = idValido(idCliente);
  sku = skuValido(sku);
  if (!Number.isInteger(nuevoMonto) || nuevoMonto < 0 || nuevoMonto > 4294967295) throw new Error('Ingresa un monto entero válido.');
  const owner = firmante(wallet);
  const { registro, cliente, venta } = cadena(nombreNegocio, owner.address, idCliente, sku);
  const ix = getCambiarMontoInstruction({ owner, registro, cliente, venta, idCliente, sku, nuevoMonto });
  return enviar(ix, wallet, connection, opciones);
}

export async function hacerDevolucion(wallet: WalletFirma, connection: Connection, nombreNegocio: string, idCliente: number, sku: string, opciones?: OpcionesEnvio): Promise<string> {
  nombreNegocio = negocioValido(nombreNegocio);
  idCliente = idValido(idCliente);
  sku = skuValido(sku);
  const owner = firmante(wallet);
  const { registro, cliente, venta } = cadena(nombreNegocio, owner.address, idCliente, sku);
  const ix = getHacerDevolucionInstruction({ owner, registro, cliente, venta, idCliente, sku });
  return enviar(ix, wallet, connection, opciones);
}