use crate::preprocessor::CommentType::{Whitespace, SlashStar, DoubleSlash};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum CommentType {
    DoubleSlash,
    SlashStar,
    String,
    ByteString,
    RawString {
        preceding_hashes: usize
    },
    //Whitespace also includes newline '\n' and '\t'
    Whitespace
}

impl CommentType {
    fn any_comment(src_text: &[char], index: usize) -> Option<CommentType> {
        match src_text[index] {
            'r' => {
                if index + 1 >= src_text.len() {
                    return None;
                }

                let mut hashes = 0;
                let mut tmp_index = index + 1;
                loop {
                    if src_text[tmp_index] == '#' {
                        hashes += 1;
                    } else {
                        break;
                    }
                    tmp_index += 1;
                    if tmp_index >= src_text.len() {
                        return None;
                    }
                }
                if src_text[tmp_index] == '"' {
                    Some(CommentType::RawString {preceding_hashes: hashes})
                } else {
                    None
                }
            },
            'b' => {
                if index + 1 < src_text.len() && src_text[index + 1] == '"' {
                    Some(CommentType::ByteString)
                } else {
                    None
                }
            }
            '"' => {
                Some(CommentType::String)
            },
            '/' => {
                if index != src_text.len() - 1 {
                    match src_text[index + 1] {
                        '*' => Some(SlashStar),
                        '/' => Some(DoubleSlash),
                        _ => None
                    }
                } else {
                    None
                }
            },
            ' ' | '\n' | '\t' => Some(Whitespace),
            _ => None
        }
    }

    fn comment_length(&self, src_text: &[char], comment_begin: usize) -> Result<usize, ()> {
        let mut tmp_index = comment_begin;
        let mut length;
        match self {
            CommentType::String => {
                length = 2; //Account for the starting and ending "
                tmp_index += 1;
                loop {
                    if tmp_index == src_text.len() {
                        return Result::Err(());
                    }
                    if src_text[tmp_index] == '"' && src_text[tmp_index - 1] != '\\' {
                        break;
                    }
                    length += 1;
                    tmp_index += 1;
                }
                Ok(length)
            },
            CommentType::DoubleSlash => {
                length = 3; //Account for the starting // and ending newline
                tmp_index += 2;
                loop {
                    if tmp_index == src_text.len() {
                        return Result::Err(());
                    }
                    if src_text[tmp_index] == '\n' {
                        break;
                    }
                    length += 1;
                    tmp_index += 1;
                }
                Ok(length)
            }
            CommentType::SlashStar => {
                length = 4; //Account for the starting /* and ending */
                tmp_index += 3;
                loop {
                    if tmp_index == src_text.len() {
                        return Result::Err(());
                    }
                    if src_text[tmp_index] == '/' && src_text[tmp_index - 1] == '*' {
                        break;
                    }
                    length += 1;
                    tmp_index += 1;
                }
                Ok(length)
            }
            CommentType::ByteString => {
                length = 4; //Account for the starting and ending ", as well as the b
                tmp_index += 1;
                loop {
                    if tmp_index == src_text.len() {
                        return Result::Err(());
                    }
                    if src_text[tmp_index] == '"' && src_text[tmp_index - 1] != '\\' {
                        break;
                    }
                    length += 1;
                    tmp_index += 1;
                }
                Ok(length)
            }
            CommentType::RawString { preceding_hashes } => {
                length = 4 + preceding_hashes * 2; //Account for the starting and ending ", as well as the r and the hashes
                tmp_index += preceding_hashes + 2;
                loop {
                    if tmp_index + preceding_hashes + 1 >= src_text.len() {
                        return Result::Err(());
                    }

                    let mut finished = true;
                    'inner: for i in tmp_index..(tmp_index + preceding_hashes) {
                        if src_text[i] != '#' {
                            finished = false;
                            break 'inner;
                        }
                    }
                    if finished {
                        break;
                    }

                    length += 1;
                    tmp_index += 1;
                }
                Ok(length)
            }
            CommentType::Whitespace => {
                length = 0;
                while tmp_index < src_text.len() &&
                    (src_text[tmp_index] == ' ' || src_text[tmp_index] == '\n' ||src_text[tmp_index] == '\t') {
                    tmp_index += 1;
                    length += 1;
                }
                Ok(length)
            }
        }
    }

    ///Returns the amount of inserted characters
    fn replace_in_source(&self, index: usize, source: &mut [char]) -> usize {
        match self {
            CommentType::DoubleSlash | SlashStar | Whitespace => {
                if index != 0 && source[index - 1] != '\n' {
                    match source[index - 1] {
                        'a'..='z' | 'A'..='Z' => {
                            source[index] = '\n';
                            1
                        }
                        _ => {
                            0
                        }
                    }
                } else {
                    0
                }
            }
            CommentType::String | CommentType::ByteString | CommentType::RawString { .. } => {
                source[index] = '"';
                source[index + 1] = '"';
                2
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate)  struct Comment {
    pub(crate) begin: usize,
    pub(crate) length: usize,
    pub(crate) comment_type: CommentType
}

pub(crate) fn preprocess(source_text: &[char], result_text: &mut [char]) -> Result<Vec<Comment>, usize> {
    let mut ret = Vec::new();
    let mut index_in_src = 0;
    let mut index_in_result = 0;
    while index_in_src < source_text.len() {
        let comment = CommentType::any_comment(source_text, index_in_src);
        match comment {
            Some(c) => {
                match c.comment_length(source_text, index_in_src) {
                    Ok(length) => {
                        ret.push(Comment {
                            begin: index_in_src,
                            length,
                            comment_type: c
                        });

                        index_in_src += length;
                        index_in_result += c.replace_in_source(index_in_result, result_text);
                    },
                    Err(()) => {
                        return Err(index_in_src);
                    }
                }

            }
            None => {
                //TODO count semicolons, blocks etc.
                result_text[index_in_result] = source_text[index_in_src];
                index_in_src += 1;
                index_in_result += 1;
            }
        }
    }

    Ok(ret)
}

/*
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum CommentType {
    DoubleSlash,
    SlashStar,
    String,
    ByteString,
    RawString {
        preceding_hashes: usize
    },
    //Whitespace also includes newline '\n' and '\t'
    Whitespace
}

#[derive(Clone, Copy, Debug)]
pub(crate)  struct Comment {
    pub(crate) begin: usize,
    pub(crate) length: usize,
    pub(crate) comment_type: CommentType
}

pub(crate) fn preprocess(source_text: &[char], result_text: &mut [char]) -> Vec<Comment> {
    let mut ret = Vec::new();

    let mut result_text_index = 0;

    let mut current_comment : Option<Comment>= Option::None;

    let mut finished = false;

    for i in 0..source_text.len() {
        //TODO The char literals such as 'a' are not yet matched, because they also server as lifetime delimiters for references
        match current_comment {
            Some(mut c) => {
                c.length += 1;
                current_comment = match c.comment_type {
                    CommentType::DoubleSlash => {
                        if c.length == 1 {
                            //Determine what kind of comment, and if it is any at all
                            if (i + 1) < source_text.len() {
                                //The slash-star comment also begins with a comment
                                match source_text[i + 1] {
                                    '*' => {
                                        c.comment_type = SlashStar;
                                        Option::Some(c)
                                    },
                                    '/' => {
                                        Option::Some(c)
                                    },
                                    _ => {
                                        //Not a comment at all, drop current_comment
                                        //Todo append previous character to list
                                        Option::None
                                    },
                                }
                            } else {
                                //Todo append previous character to list
                                //If this was the last character in the document, there is no comment
                                Option::None
                            }
                        } else {
                            //Determine if the comment ends
                            //No bounds check, it is guaranteed that there is a preceding character
                            if source_text[i] == '\n' {
                                finished = true;
                            }
                            Option::Some(c)
                        }
                    },
                    SlashStar => {
                        //Check if comment ends
                        if source_text[i] == '/' && source_text[i - 1] == '*' {
                            finished = true;
                        }
                        Option::Some(c)
                    },
                    CommentType::String | CommentType::ByteString => {
                        //TODO process escaped characters
                        if source_text[i] == '"' && source_text[i - 1] != '\\' {
                            finished = true;
                        }
                        Option::Some(c)
                    },
                    CommentType::RawString { preceding_hashes } => {
                        if source_text[i] == '#' {
                            let mut correct_hashes = true;
                            for k in 1..preceding_hashes {
                                if source_text[i - k] != '#' {
                                    correct_hashes = false;
                                }
                            }
                            if correct_hashes && i > preceding_hashes &&
                                    source_text[i - preceding_hashes - 1] == '"' {
                                finished = true;
                            }
                        }
                        Option::Some(c)
                    },
                    CommentType::Whitespace => {
                        //TODO the dropped character here could be the begin of a new comment
                        if source_text[i] != ' ' && source_text[i] != '\n' && source_text[i] != '\t' {
                            finished = true;
                        }
                        Option::Some(c)
                    },
                };
            },
            None => {
                current_comment = match source_text[i] {
                    '/' => {
                        //The slash-star and slash-slash comment both are marked as DoubleSlash
                        //At the beginning. This is corrected in the next cycle
                        Option::Some(Comment {
                            begin: i,
                            length: 1,
                            comment_type: CommentType::DoubleSlash
                        })
                    },
                    '"' => {
                        if i > 0 {
                            match source_text[i - 1] {
                                'b' => {
                                    Option::Some(Comment {
                                        begin: i - 1,
                                        length: 2,
                                        comment_type: CommentType::ByteString
                                    })
                                },
                                '#' => {
                                    let mut found = 0;
                                    if i != 0  {
                                        let mut k = i - 1;
                                        loop {
                                            if source_text[k] == '#' {
                                                found += 1;
                                            } else {
                                                break;
                                            }
                                            if k == 0 {
                                                break;
                                            }
                                            k -= 1;
                                        }
                                        if source_text[k] != 'r' {
                                            //No 'r' in front of the hash
                                            Option::Some(Comment {
                                                begin: i,
                                                length: 1,
                                                comment_type: CommentType::String
                                            })
                                        } else {
                                            Option::Some(Comment {
                                                begin: i - 1 - found,
                                                length: 2 + found,
                                                comment_type: CommentType::RawString {
                                                    preceding_hashes: found
                                                }
                                            })
                                        }
                                    } else {
                                        //No 'r' in front of the hash
                                        Option::Some(Comment {
                                            begin: i,
                                            length: 1,
                                            comment_type: CommentType::String
                                        })
                                    }
                                },
                                'r' => {
                                    Option::Some(Comment {
                                        begin: i - 1,
                                        length: 2,
                                        comment_type: CommentType::RawString { preceding_hashes: 0}
                                    })
                                },
                                _ => Option::Some(Comment {
                                    begin: i,
                                    length: 1,
                                    comment_type: CommentType::String
                                })
                            }
                        } else {
                            Option::Some(Comment {
                                begin: i,
                                length: 1,
                                comment_type: CommentType::String
                            })
                        }
                    }
                    '\n' | ' ' | '\t' => {
                        Option::Some(Comment {
                            begin: i,
                            //The length is incremented one time too much at the end,
                            // so we already decrement it here
                            length: 0,
                            comment_type: CommentType::Whitespace
                        })
                    }
                    _ => {
                        Option::None
                    }
                };
            },
        }

        if current_comment.is_none() {
            //TODO remember if it is the start of block or a semicolon
            result_text[result_text_index] = source_text[i];
            result_text_index += 1;
        } else if current_comment.unwrap().comment_type == Whitespace && i == source_text.len() - 1{
            //Whitespace-Comments at the end of a file have to end
            //Length incrementation is needed because of sluggish code written before
            let mut tmp = current_comment.unwrap();
            tmp.length += 1;
            current_comment = Option::Some(tmp);
            finished = true;
        }

        if finished {
            //todo replace unwrap with ? to forward error
            let comment = current_comment.unwrap();

            //Add a newline to the resulting text
            if result_text_index == 0 || result_text[result_text_index - 1] != '\n' {
                result_text[result_text_index] = '\n';
                result_text_index += 1;
            }

            //If the whitespace comment was finished in this cycle, the finishing character has to saved
            if comment.comment_type == Whitespace {
                result_text[result_text_index] = source_text[i];
                result_text_index += 1;
            }

            current_comment = Option::None;
            ret.push(comment);
            finished = false;
        }
    }
    assert!(current_comment.is_none());
    ret
}
*/

/*
pub(crate) fn find_pairs_cpu(text: &[u8]) -> Vec<cl_part::CommentPair> {
    let mut ret= Vec::new();

    let mut comment: u8 = 0;
    let mut start: usize = 0;
    for i in 1..text.len() {
        match comment {
            1 => if text[i] as char == '\n' {
                ret.push(cl_part::CommentPair {
                    begin: start,
                    end: i,
                    t: 1
                });
                //Next one cannot start a comment with this token as the previous
                comment = 4;
            },
            2 => if text[i] as char == '/' &&
                text[i - 1] as char == '*' {
                ret.push(cl_part::CommentPair {
                    begin: start,
                    end: i,
                    t: 2
                });
                comment = 4;
            },
            3 => if text[i] as char == ')' &&
                text[i - 1] as char == '"' {
                ret.push(cl_part::CommentPair {
                    begin: start,
                    end: i,
                    t: 3
                });
                comment = 4;
            },
            4 => {
                comment = 0;
                //Do nothing, skip
            },
            _ => match text[i] as char {
                '/' => {
                    comment = if text[i - 1] as char == '/' {
                        start = i - 1;
                        1
                    } else {
                        0
                    }
                },
                '*' => {
                    comment = if text[i - 1] as char == '/' {
                        start = i - 1;
                        2
                    } else {
                        0
                    }
                },
                '"' => {
                    comment = if text[i - 1] as char == '(' {
                        start = i - 1;
                        3
                    } else {
                        0
                    }
                }
                _ => (),
            },
        };

    }

    ret
}
*/
