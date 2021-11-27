import React from 'react';
import logo from './logo.svg';
import './App.css';
import TextField from '@material-ui/core/TextField'
import Container from '@material-ui/core/Box'
import Divider from '@material-ui/core/Divider'
import * as Layout from './Layout'

import { LAMPORTS_PER_SOL,Account, PublicKey, Connection, SystemProgram ,Transaction,sendAndConfirmTransaction} from '@solana/web3.js';
import { Button,Grid } from '@material-ui/core';
import { HelloWorld } from './HelloWorld';



class Content extends React.Component {

  constructor(props) {
    super(props)
    this.state = { };
    this.onErase = this.onErase.bind(this);
    this.onHello = this.onHello.bind(this);
    this.onQuery = this.onQuery.bind(this);

    //let url =  'http://api.mainnet-beta.solana.com';
    //let url =  'https://solana-api.projectserum.com';
    //let url =  'https://devnet.solana.com';
    let url =  'http://47.242.205.48:8899';
    this.programID = new PublicKey("BnUch1E847xL9jMixiNBC3QCNx3jBfg1SJy3cM5nnjfU");
    this.playerPrivKey = [158,188,223,83,176,236,248,185,74,218,97,26,205,49,90,0,188,33,169,26,210,102,99,189,41,132,175,250,234,212,170,189,160,59,58,253,225,203,106,91,111,71,182,42,92,242,86,172,148,95,157,9,12,216,141,50,80,1,31,223,176,114,86,119];
    this.connection = new Connection(url);
    this.messageAccount = new Account();
    this.playerAccount = new Account(this.playerPrivKey);
  }


  render() {
    return (
      <Container>


        <React.Fragment>
          <Button onClick={this.onHello}> hello</Button>
        </React.Fragment>
        <Divider />
        <React.Fragment>
          <Button onClick={this.onErase}> erase</Button>
        </React.Fragment>
        <Divider />
        <React.Fragment>
          <Button onClick={this.onQuery}> query</Button>
        </React.Fragment>
      </Container>
    );
  }

  async onQuery() {

  }

  async onErase() {
    let trxi = HelloWorld.createEraseInstruction(
      this.playerAccount.publicKey,
      this.messageAccount.publicKey,
      this.programID,
    );

    const transaction = new Transaction();
    transaction.add(trxi);

    let signers= [this.playerAccount, this.messageAccount];
    sendAndConfirmTransaction(this.connection, transaction, signers, {
        skipPreflight: false,
        commitment: 'recent',
        preflightCommitment: 'recent',
    }).then(()=>{
      console.log("done erase");
    }).catch((e)=>{
      console.log("error:", e);
    })
  }

  async onHello() {

    let messageNeeded = await this.connection.getMinimumBalanceForRentExemption(Layout.messagSpace);

    const trxi0 =  SystemProgram.createAccount({
      fromPubkey: this.playerAccount.publicKey,
      newAccountPubkey: this.messageAccount.publicKey,
      lamports: messageNeeded,
      space: Layout.messagSpace,
      programId: this.programID,
    });

    console.log("message:", this.messageAccount.publicKey.toBase58());


    let trxi = HelloWorld.createHelloInstruction(
      this.playerAccount.publicKey,
      this.messageAccount.publicKey,
      this.programID,
      "hello world!",
    );

    const transaction = new Transaction();
    transaction.add(trxi0);
    transaction.add(trxi);

    let signers= [this.playerAccount, this.messageAccount];
    sendAndConfirmTransaction(this.connection, transaction, signers, {
        skipPreflight: false,
        commitment: 'recent',
        preflightCommitment: 'recent',
    }).then(()=>{
      console.log("done hello");
    }).catch((e)=>{
      console.log("error:", e);
    })

  }
}


function App() {
  return (
    <Content />
  );
}

export default App;