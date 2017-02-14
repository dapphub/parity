// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

import { action, observable } from 'mobx';

import Ledger from '~/3rdparty/ledger';

const HW_SCAN_INTERVAL = 5000;
let instance = null;

export default class HardwareStore {
  @observable isScanning = false;
  @observable wallet = null;

  constructor (api) {
    this._api = api;
    this._ledger = Ledger.create();
    this._pollId = null;

    this.pollScan();
  }

  @action setScanning = (isScanning) => {
    this.isScanning = isScanning;
  }

  @action setWallet = (wallet) => {
    this.wallet = wallet;
  }

  scanLedger () {
    return this._ledger
      .scan()
      .then((wallet) => {
        console.log('HardwareStore::scanLedger', wallet);
      })
      .catch((error) => {
        console.warn('HardwareStore::scanLedger', error);
      });
  }

  scanParity () {
    return this._api.parity
      .hardwareAccountsInfo()
      .then((accountsInfo) => {
        console.log('HardwareStore::scanParity', accountsInfo);
      })
      .catch((error) => {
        console.warn('HardwareStore::scanParity', error);
      });
  }

  scan () {
    this.setScanning(true);

    // NOTE: Depending on how the harware is configured and how the local env setup
    // is done, different results will be retrieved via Parity vs. the browser APIs
    // (latter is Chrome-only, needs the browser app enabled on a Ledger, former is
    // not intended as a network call, i.e. hw wallet is with the user)
    return Promise
      .all([
        this.scanLedger(),
        this.scanParity()
      ])
      .then(() => {
        this.setScanning(false);
      });
  }

  createEntry (entry) {
    return Promise
      .all([
        this._api.parity.setAccountName(entry.address, entry.name),
        this._api.parity.setAccountMeta(entry.address, {
          deleted: false,
          description: entry.description,
          hardware: {
            type: entry.type
          },
          name: entry.name,
          tags: ['hardware'],
          timestamp: Date.now()
        })
      ])
      .catch((error) => {
        console.warn('HardwareStore::createEntry', error);
        throw error;
      });
  }

  pollScan = () => {
    this._pollId = setTimeout(() => {
      this.scan().then(this.pollScan);
    }, HW_SCAN_INTERVAL);
  }

  static get () {
    if (!instance) {
      instance = new HardwareStore();
    }

    return instance;
  }
}

export {
  HW_SCAN_INTERVAL
};