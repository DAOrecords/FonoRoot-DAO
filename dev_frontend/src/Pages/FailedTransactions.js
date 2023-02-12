import { utils } from 'near-api-js';
import React, { useState, useEffect } from 'react';
import Nav from '../Nav';
import { actOnProposal, getLastProposalId, getListOfFailedTransactions, resendFailedTransaction } from '../utils';


export default function FailedTransactions() {
  const [failedTransactions, setFailedTransactions] = useState([]);
  const [newAddress, setNewAddress] = useState("");
  const [selected, setSelected] = useState(null);
  const [readyForVote, setReadyForVote] = useState(false);
  const [message, setMessage] = useState("");
  const [proposalId, setProposalId] = useState(null);

  // Get the list of IncomeTables
  useEffect(async () => {
    const list = await getListOfFailedTransactions(0, 1000);
    setFailedTransactions(list);
  }, []);

  async function initiateResend() {
    if (selected === null) {
      console.log("Nothing is selected.");
      return;
    }
  
    const lastProposalId = await getLastProposalId();
    localStorage.setItem("last_proposal_id", lastProposalId);
    
    const returnedProposalID = await resendFailedTransaction(selected, newAddress);
    localStorage.setItem("last_proposal_id", returnedProposalID);
    setProposalId(returnedProposalID);
    setReadyForVote(true);
  }

  // Act on proposal for Failed Transaction
  async function finaliizePayout() {
    console.log(proposalId)
    const returnMessage = await actOnProposal(proposalId);
    setMessage(JSON.stringify(returnMessage));
  }

  function formatNumber(inputNumber) {
    console.log("inputNumber: ", inputNumber);
    //const stringNumber = inputNumber.toString(10);
    //utils.format.formatNearAmount(inputNumber);
    // can't solve this
    return inputNumber;
  }


  return (
    <>
      <Nav />
      <main>
        <h2>Failed Transactions</h2>

        <section>
          {"Select a failed transaction to resend it. Only Council members are allowed to do this."}
          <ul className="revenueList">
            {failedTransactions.map((FailedTransaction) => (
              <li 
                key={FailedTransaction[0]}
                onClick={() => setSelected(FailedTransaction[0])}
                className="resendElement"
              >
                <p>{"Failed ID: "}{FailedTransaction[0]}</p>
                <p>{"Beneficiary: "}{FailedTransaction[1].beneficiary}</p>
                <p>{"Amount: "}{formatNumber(FailedTransaction[1].amount)}{" yoctoNEAR"}</p>
              </li>
            ))}
          </ul>
        </section>

        <section>
          <p>{"Selected: "}{selected}</p>
          <label htmlFor=""></label>
          <input type={"text"} placeholder={"alice.testnet"} value={newAddress} onChange={(e) => setNewAddress(e.target.value)}></input>
          <button onClick={initiateResend}>{"Initiate Resend Failed Transaction"}</button>
        </section>

        <section>
          {readyForVote && <p className="finalizeMessage">You can click on finalize now!</p>}
          
          <button onClick={finaliizePayout}>{"Finalize Resend Failed Transaction"}</button>
          <p><code>{message}</code></p>
        </section>
      </main>
    </>
  )
}
