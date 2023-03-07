use anchor_lang::prelude::*;
use std::time::{Duration, SystemTime};

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("11111111111111111111111111111111");

#[program]
mod hello_anchor {
    use super::*;
           
    pub fn create_proposal(ctx: Context<Create_Proposal>,pool_prize: u64,days_b4_contest : u64,high_rish_value: u8,medium_risk_value: u8) -> Result<()>{
        let min_amount = 10000;
        require!(pool_prize>min_amount);

        //a minimum time is to be decided upon after acception
          
        //currently gets current time
        let start_time = ctx.accounts.clock.unix_timestamp +  days_b4_contest* 24 * 60 * 60;
        let end_date = start_time + contest_duration * 24 * 60 * 60;   
          
        let judge_cut = 0.1*pool_prize;
        let DAO_cut = 0.05*pool_prize;

        // Variable cuts for vulnerabilities
        //should add up to 100
        //value taken in percent/100  
        let high_risk_vulnerability_percent = high_risk_value;
        let high_risk_pool = high_risk_vulnerability_percent*pool_prize;
        let medium_risk_vulnerability_percent = medium_risk_value; 
        let medium_risk_pool = medium_risk_vulnerability_percent*pool_prize;
        let gas_report_and_low_risk_cut = (0.85 - high_risk_vulnerability_percent -  medium_risk_vulnerability_percent)*pool_prize;

        //stake 25 % of the pool prize 
        let stake = 0.25*pool_prize;
        // where to put the stake ? will the contract have it ?
        let proposal = Proposal {
        authority: ctx.accounts.authority.clone(),
        governance_token_account: ctx.accounts.governance_token_account.clone(),
        proposal_account: ctx.accounts.proposal_account.clone(),
        title,
        start_date,
        end_date,
        prize_pool,
        };

        Ok(());
      }

    pub fn vote_for_proposal(ctx: Context<Vote_For_Proposal>,vote_type: VoteType) -> Result<()>{
    let vote_account = &mut ctx.accounts.vote_account;
    let governance_token_account = &mut ctx.accounts.governance_token_account;
    let voter = &ctx.accounts.voter;

    // Check if the governance token is owned by the program.
    if governance_token_account.owner != *ctx.program_id {
        return Err(ErrorCode::NotProgramToken.into());
    }

    // Check if the voter has enough tokens to cast a vote.
    let voter_balance = governance_token_account.balance;
    if voter_balance == 0 {
        return Err(ErrorCode::InsufficientTokens.into());
    }

    //no proposal struct yet
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let voting_period = Duration::from_secs(2 * 24 * 60 * 60); // 2 days
    let voting_start_time = proposal.voting_start_time;
    let voting_end_time = voting_start_time + voting_period;
        
    if current_time < voting_start_time || current_time > voting_end_time {
        return Err(ErrorCode::VotingPeriodOver.into());
    }
            match vote_type {
            VoteType::YES => {
                msg!("Voted for YES 🤝");
                 let vote_weight = voter_balance;
                 vote_account.yes += vote_weight; 
            },
            VoteType::NO => {
                msg!("Voted for NO 🤞");
                 let vote_weight = voter_balance;
                 vote_account.no += vote_weight;
            },
        };
        Ok(())

          // delegation of power ??
      }

      pub fn voting_verdict(ctx: <Voting_Verdict>) -> Result<()>{
        
        // need to create proposal id with proposal attribute
        if(yes_votes > 0.66*total_votes_casted && total_votes_casted > 0.10*totalsupply){
          proposal_id.eligible = true ;
        }else{
          proposal_id.eligible = false ;
        }
          Ok(());
      }
     
      pub fn start_contest(ctx: Context<Start_Contest>) -> Result<()>{
          // bind the proposal id with specific user so as to act as owneer of proposal
        let stake_left = 0.75*prize_pool;

          //add codebase
      }

      pub fn apply_for_judge(ctx: Context<Apply_For_Judge>) -> Result<()>{
          let candidate = new Candidate {
             name,
             email,
             proposal,
             //votes are to be initialised to zero,
             candidate_id,
          };
      }

      pub fn vote_for_judge(ctx: Context<Vote_For_Judge>,choice1: u64,choice2: u64,choice3: u64) -> Result<()>{
    
    let vote_account = &mut ctx.accounts.vote_account;
    let governance_token_account = &mut ctx.accounts.governance_token_account;
    let voter = &ctx.accounts.voter;

    // Check if the governance token is owned by the program.
    if governance_token_account.owner != *ctx.program_id {
        return Err(ErrorCode::NotProgramToken.into());
    }

    // Check if the voter has enough tokens to cast a vote.
    let voter_balance = governance_token_account.balance;
    if voter_balance == 0 {
        return Err(ErrorCode::InsufficientTokens.into());
    }

    //no proposal struct yet
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let voting_period = Duration::from_secs(2 * 24 * 60 * 60); // 2 days
    let voting_start_time = proposal.voting_start_time;
    let voting_end_time = voting_start_time + voting_period;
        
    if current_time < voting_start_time || current_time > voting_end_time {
        return Err(ErrorCode::VotingPeriodOver.into());
    }
        // voting judges based on priority 
        // not properly implemented right now
         let vote_weight = voter_balance;
          choice1.votes += 5*vote_weight;
          choice2.votes += 3*vote_weight;
          choice3.votes += 1*vote_weight;
          
        Ok(())
       
      }
    
    pub fn get_candidates(
        _ctx: Context<GetCandidates>,
    ) -> ProgramResult {
         let accounts = _ctx.accounts;

    // Iterate through the list of candidate accounts and retrieve the number of votes for each candidate.
    let mut candidate_votes: Vec<(Candidate, u32)> = accounts
        .candidate_list
        .to_account_infos()
        .into_iter()
        .map(|info| (Candidate::unpack(&info.data.borrow()).unwrap(), info.data.borrow().len() as u32))
        .collect();

    // Sort the list of candidates based on the number of votes they have received, in descending order.
    candidate_votes.sort_by(|a, b| b.1.cmp(&a.1));
    let my_vec: Vec<string> = vec![""; 10];
    // Print the top three candidates based on the number of votes they have received.
    // println!("Top three candidates:");
     for (i, (candidate, votes)) in candidate_votes.iter().enumerate().take(3) {
        // println!("{}: {:?} ({} votes)", i + 1, candidate, votes);
        judge[i]=candidate.name;
     }
     
    }
     

      pub fn submit_report(ctx: Context<Submit_Report>, report_hash: ) -> Result<()>{
         // function to add the report on blockchain 
         // hash of the data inserted and the contest id 
         // both are bind together
         

         //distribution of prizes to be done by this function too
         // 75 % to be distributed equally rest by ranking
         // different for different risk ratings
     }

      pub fn propose_report(ctx:Context<Propose_Report>) -> Result<()>{
         let mut High_risk_rewardees: Vec<Vec<u32>> = Vec::new();
         let mut Medium_risk_rewardees: Vec<Vec<u32>> = Vec::new();
         let mut Report_rewardees: Vec<Vec<u32>> = Vec::new();

        // how are we getting the user input ??
          
      }

      pub fn vote_for_slash(ctx:Context<Vote_For_Slash>) -> Result<()>{
          // if a malicious activity by judges found by a member , slash the stakes of the judge and dont give their prize pool
          // prize pool distributed to dao
      }
}

#[derive(Accounts)]
pub struct Proposal {
    //proposal by the protocol
    pub authority: Signer<'info>,
    pub governance_token_account: Account<'info, GovernanceTokenAccount>,
    pub proposal_account: Account<'info, ProposalAccount>,
    pub title: String,
    pub start_date: String,
    pub end_date: String,
    pub prize_pool: u64,
}

#[derive(Accounts)]
pub struct Candidate {
    pub name: String,
    pub email: String,
    pub proposal: String,
    pub votes: u64,
    pub candidate_id: u64,
}

pub struct GetCandidates<'info> {
    // to get the list of candidates
    #[account(address = "")]
    pub candidate_list: AccountInfo<'info>,
}

pub struct Create_Proposal<'info> {
 clock: Sysvar<'info, Clock>,
}
                            
pub struct Vote_For_Proposal<'info> {
   
    #[account(has_one = governance_token)]
    governance_token_account: Account<'info, GovernanceToken>,
    // Storing Votes in global account
    #[account(mut)] 
    pub vote_account: Account<'info, VoteBank>,
    pub signer: Signer<'info>,

}

pub struct Get_Votes<'info>{
    #[account(has_one = governance_token)]
    governance_token_account: Account<'info, GovernanceToken>,
    #[account(has_one = vote_account)]
    vote_account: Account<'info, VoteBank>,
}

pub struct Start_Contest<'info> {
    
}
pub struct Apply_For_Judge<'info> {
    
}
pub struct Vote_For_Judge<'info> {
    
}
pub struct Submit_Report<'info> {
    
}
pub struct Vote_For_Slash<'info> {
    
}


#[account]
pub struct GovernanceToken {
    pub owner: Pubkey,
    pub balance: u64,
}
#[derive(Default)]
pub struct VoteBank {
    yes: u64, // 8 bytes in size
    no: u64, // 8 bytes in size
}
#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum VoteType {
    YES,
    NO
}

#[error]
pub enum ErrorCode {
    #[msg("The proposal is no longer in its voting period")]
    VotingPeriodOver,
    #[msg("Insufficient tokens")]
    InsufficientTokens,
    #[msg("Not a program-owned governance token")]
    NotProgramToken,
}